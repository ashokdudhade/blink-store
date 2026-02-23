//! In-memory engine with sampled eviction and size tracking.
//!
//! Replaces global `Mutex<LruCache>` with a lock-free sampled-eviction
//! strategy: each entry stores a monotonic access counter, and on eviction
//! we sample a handful of entries and evict the one with the lowest counter.

use crate::error::BlinkError;
use bytes::Bytes;
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::trace;

/// Abstraction layer for storage backends.
pub trait BlinkStorage: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<Bytes>, BlinkError>;
    fn set(&self, key: &str, value: Bytes) -> Result<(), BlinkError>;
    fn delete(&self, key: &str) -> Result<bool, BlinkError>;
    fn current_usage_bytes(&self) -> Result<u64, BlinkError>;
}

fn entry_size(key: &str, value: &[u8]) -> u64 {
    (key.len() + value.len()) as u64
}

const EVICTION_SAMPLES: usize = 5;

/// In-memory engine with sampled eviction and memory-cap enforcement.
pub struct MemoryEngine {
    store: DashMap<String, (Bytes, u64)>,
    limit_bytes: u64,
    current_usage: AtomicU64,
    access_counter: AtomicU64,
}

impl MemoryEngine {
    pub fn new(limit_bytes: u64) -> Result<Self, BlinkError> {
        Ok(Self {
            store: DashMap::new(),
            limit_bytes,
            current_usage: AtomicU64::new(0),
            access_counter: AtomicU64::new(0),
        })
    }

    fn next_counter(&self) -> u64 {
        self.access_counter.fetch_add(1, Ordering::Relaxed)
    }

    /// Evicts entries with the lowest access counter until there is room
    /// for `need_bytes` additional bytes.
    fn evict_until_room(&self, need_bytes: u64) {
        while self.current_usage.load(Ordering::Acquire) + need_bytes > self.limit_bytes {
            let mut victim_key: Option<String> = None;
            let mut victim_counter = u64::MAX;
            let mut sampled = 0;

            for entry in self.store.iter() {
                if sampled >= EVICTION_SAMPLES {
                    break;
                }
                let counter = entry.value().1;
                if counter < victim_counter {
                    victim_key = Some(entry.key().clone());
                    victim_counter = counter;
                }
                sampled += 1;
            }

            match victim_key {
                Some(key) => {
                    if let Some((_, (value, _))) = self.store.remove(&key) {
                        let freed = entry_size(&key, &value);
                        self.current_usage.fetch_sub(freed, Ordering::Release);
                        trace!(key = %key, freed_bytes = freed, "evicted");
                    }
                }
                None => break,
            }
        }
    }
}

impl BlinkStorage for MemoryEngine {
    fn get(&self, key: &str) -> Result<Option<Bytes>, BlinkError> {
        if let Some(mut entry) = self.store.get_mut(key) {
            entry.value_mut().1 = self.next_counter();
            Ok(Some(entry.value().0.clone()))
        } else {
            Ok(None)
        }
    }

    fn set(&self, key: &str, value: Bytes) -> Result<(), BlinkError> {
        let need = entry_size(key, &value);
        let old_size = self
            .store
            .get(key)
            .map(|e| entry_size(key, &e.value().0))
            .unwrap_or(0);

        let current = self.current_usage.load(Ordering::Acquire);
        if current + need - old_size > self.limit_bytes {
            self.evict_until_room(need.saturating_sub(old_size));
        }

        let counter = self.next_counter();
        self.store.insert(key.to_owned(), (value, counter));
        self.current_usage.fetch_add(need, Ordering::Release);
        if old_size > 0 {
            self.current_usage.fetch_sub(old_size, Ordering::Release);
        }
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<bool, BlinkError> {
        if let Some((_, (value, _))) = self.store.remove(key) {
            let freed = entry_size(key, &value);
            self.current_usage.fetch_sub(freed, Ordering::Release);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn current_usage_bytes(&self) -> Result<u64, BlinkError> {
        Ok(self.current_usage.load(Ordering::Acquire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(limit_bytes: u64) -> MemoryEngine {
        MemoryEngine::new(limit_bytes).unwrap()
    }

    #[test]
    fn get_missing_returns_none() {
        let e = engine(1024);
        assert!(e.get("missing").unwrap().is_none());
    }

    #[test]
    fn set_and_get() {
        let e = engine(1024);
        e.set("k", Bytes::from_static(b"hello")).unwrap();
        let v = e.get("k").unwrap().unwrap();
        assert_eq!(&v[..], b"hello");
    }

    #[test]
    fn delete_removes_key() {
        let e = engine(1024);
        e.set("k", Bytes::from_static(b"v")).unwrap();
        assert!(e.delete("k").unwrap());
        assert!(e.get("k").unwrap().is_none());
    }

    #[test]
    fn delete_missing_returns_false() {
        let e = engine(1024);
        assert!(!e.delete("missing").unwrap());
    }

    #[test]
    fn current_usage_tracks_bytes() {
        let e = engine(1024);
        assert_eq!(e.current_usage_bytes().unwrap(), 0);
        e.set("a", Bytes::from_static(b"xx")).unwrap();
        assert_eq!(e.current_usage_bytes().unwrap(), 3);
        e.set("ab", Bytes::from_static(b"y")).unwrap();
        assert_eq!(e.current_usage_bytes().unwrap(), 6);
        e.delete("a").unwrap();
        assert_eq!(e.current_usage_bytes().unwrap(), 3);
    }

    #[test]
    fn eviction_when_over_limit() {
        let e = engine(10);
        e.set("a", Bytes::from_static(b"111")).unwrap(); // 1+3=4
        e.set("b", Bytes::from_static(b"2222")).unwrap(); // 1+4=5, total 9
        e.set("c", Bytes::from_static(b"x")).unwrap(); // 1+1=2, would be 11 -> evict oldest
        assert!(e.get("a").unwrap().is_none()); // "a" has lowest counter
        assert_eq!(&e.get("b").unwrap().unwrap()[..], b"2222");
        assert_eq!(&e.get("c").unwrap().unwrap()[..], b"x");
    }

    #[test]
    fn replace_key_updates_usage() {
        let e = engine(20);
        e.set("k", Bytes::from_static(b"aaa")).unwrap();
        e.set("k", Bytes::from_static(b"bb")).unwrap();
        assert_eq!(e.current_usage_bytes().unwrap(), 3);
        assert_eq!(&e.get("k").unwrap().unwrap()[..], b"bb");
    }

    #[test]
    fn trait_interface() {
        let store: std::sync::Arc<dyn BlinkStorage> =
            std::sync::Arc::new(engine(1024));
        store.set("trait_key", Bytes::from_static(b"trait_value")).unwrap();
        let v = store.get("trait_key").unwrap().unwrap();
        assert_eq!(&v[..], b"trait_value");
        assert!(store.delete("trait_key").unwrap());
    }
}
