//! In-memory engine implementing `BlinkStorage` with LRU eviction and size tracking.

use crate::error::BlinkError;
use dashmap::DashMap;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use tracing::{info, instrument};

/// Abstraction layer for storage backends (e.g. In-Memory, future Redis).
#[async_trait::async_trait]
pub trait BlinkStorage: Send + Sync {
    /// Returns the value for `key`, or `None` if not found.
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, BlinkError>;

    /// Stores `value` at `key`. May evict LRU entries if over memory limit.
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), BlinkError>;

    /// Removes `key`. Returns `true` if the key was present.
    async fn delete(&self, key: &str) -> Result<bool, BlinkError>;

    /// Current total size of stored entries in bytes.
    async fn current_usage_bytes(&self) -> Result<u64, BlinkError>;
}

/// Size in bytes for a stored entry: key bytes + value bytes.
fn entry_size(key: &str, value: &[u8]) -> u64 {
    (key.len() + value.len()) as u64
}

/// In-memory engine with LRU eviction and memory-cap enforcement.
pub struct MemoryEngine {
    /// Key -> value storage.
    store: DashMap<String, Vec<u8>>,
    /// LRU order of keys (used for eviction only; may contain stale keys after delete).
    lru: Mutex<LruCache<String, ()>>,
    /// Maximum total size in bytes.
    limit_bytes: u64,
    /// Current total size in bytes.
    current_usage: AtomicU64,
}

impl MemoryEngine {
    /// Max number of keys to track in LRU order (eviction only; we enforce by bytes).
    const LRU_KEY_CAP: usize = 1_000_000;

    /// Creates a new engine with the given memory limit in bytes.
    pub fn new(limit_bytes: u64) -> Result<Self, BlinkError> {
        let cap = NonZeroUsize::new(Self::LRU_KEY_CAP)
            .ok_or_else(|| BlinkError::Internal("LRU_KEY_CAP must be non-zero".into()))?;
        Ok(Self {
            store: DashMap::new(),
            lru: Mutex::new(LruCache::new(cap)),
            limit_bytes,
            current_usage: AtomicU64::new(0),
        })
    }

    /// Evicts least-recently-used entries until `current_usage + need_bytes <= limit_bytes`.
    fn evict_until_room(&self, need_bytes: u64) -> Result<(), BlinkError> {
        let mut lru = self.lru.lock().map_err(|e| {
            BlinkError::Internal(format!("lru lock poisoned: {}", e))
        })?;
        let mut current = self.current_usage.load(Ordering::Acquire);
        let target = self.limit_bytes.saturating_sub(need_bytes);

        while current > target {
            let key = match lru.pop_lru() {
                Some((k, _)) => k,
                None => break,
            };
            if let Some((_, old_value)) = self.store.remove(&key) {
                let freed = entry_size(&key, &old_value);
                current = self
                    .current_usage
                    .fetch_sub(freed, Ordering::Release) - freed;
                info!(key = %key, action = "evicted", freed_bytes = freed);
            }
        }

        if self.current_usage.load(Ordering::Acquire) + need_bytes > self.limit_bytes {
            return Err(BlinkError::AtCapacity);
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl BlinkStorage for MemoryEngine {
    #[instrument(skip(self))]
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, BlinkError> {
        let value = self.store.get(key).map(|r| r.clone());
        if value.is_some() {
            let mut lru = self.lru.lock().map_err(|e| {
                BlinkError::Internal(format!("lru lock poisoned: {}", e))
            })?;
            let _ = lru.get(key); // touch for LRU
            info!(key = %key, action = "get");
        }
        Ok(value)
    }

    #[instrument(skip(self, value))]
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), BlinkError> {
        let need = entry_size(key, &value);
        let old_size = self
            .store
            .get(key)
            .map(|v| entry_size(key, v.as_slice()))
            .unwrap_or(0);
        let current = self.current_usage.load(Ordering::Acquire);
        let after_set = current + need - old_size;

        if after_set > self.limit_bytes {
            let extra_needed = need.saturating_sub(old_size);
            self.evict_until_room(extra_needed)?;
        }

        self.store.insert(key.to_string(), value);
        self.current_usage.fetch_add(need, Ordering::Release);
        if old_size > 0 {
            self.current_usage.fetch_sub(old_size, Ordering::Release);
        }

        let mut lru = self.lru.lock().map_err(|e| {
            BlinkError::Internal(format!("lru lock poisoned: {}", e))
        })?;
        lru.put(key.to_string(), ());
        info!(key = %key, action = "set", size_bytes = need);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn delete(&self, key: &str) -> Result<bool, BlinkError> {
        if let Some((_, old_value)) = self.store.remove(key) {
            let freed = entry_size(key, &old_value);
            self.current_usage.fetch_sub(freed, Ordering::Release);
            info!(key = %key, action = "delete", freed_bytes = freed);
            return Ok(true);
        }
        Ok(false)
    }

    async fn current_usage_bytes(&self) -> Result<u64, BlinkError> {
        Ok(self.current_usage.load(Ordering::Acquire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(limit_bytes: u64) -> MemoryEngine {
        MemoryEngine::new(limit_bytes).unwrap()
    }

    #[tokio::test]
    async fn get_missing_returns_none() {
        let e = engine(1024);
        let v = e.get("missing").await.unwrap();
        assert!(v.is_none());
    }

    #[tokio::test]
    async fn set_and_get() {
        let e = engine(1024);
        e.set("k", b"hello".to_vec()).await.unwrap();
        let v = e.get("k").await.unwrap().unwrap();
        assert_eq!(v, b"hello");
    }

    #[tokio::test]
    async fn delete_removes_key() {
        let e = engine(1024);
        e.set("k", b"v".to_vec()).await.unwrap();
        let ok = e.delete("k").await.unwrap();
        assert!(ok);
        assert!(e.get("k").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn delete_missing_returns_false() {
        let e = engine(1024);
        let ok = e.delete("missing").await.unwrap();
        assert!(!ok);
    }

    #[tokio::test]
    async fn current_usage_tracks_bytes() {
        let e = engine(1024);
        assert_eq!(e.current_usage_bytes().await.unwrap(), 0);
        e.set("a", b"xx".to_vec()).await.unwrap(); // key "a" = 1, value = 2 -> 3
        assert_eq!(e.current_usage_bytes().await.unwrap(), 3);
        e.set("ab", b"y".to_vec()).await.unwrap(); // key "ab" = 2, value = 1 -> 3, total 6
        assert_eq!(e.current_usage_bytes().await.unwrap(), 6);
        e.delete("a").await.unwrap();
        assert_eq!(e.current_usage_bytes().await.unwrap(), 3);
    }

    #[tokio::test]
    async fn eviction_when_over_limit() {
        let e = engine(10); // 10 bytes total
        e.set("a", b"111".to_vec()).await.unwrap(); // 1+3=4
        e.set("b", b"2222".to_vec()).await.unwrap(); // 1+4=5, total 9
        e.set("c", b"x".to_vec()).await.unwrap(); // 1+1=2, total 11 > 10 -> evict a (4), then 5+2=7 ok
        assert!(e.get("a").await.unwrap().is_none());
        assert_eq!(e.get("b").await.unwrap().unwrap(), b"2222");
        assert_eq!(e.get("c").await.unwrap().unwrap(), b"x");
    }

    #[tokio::test]
    async fn replace_key_updates_usage() {
        let e = engine(20);
        e.set("k", b"aaa".to_vec()).await.unwrap(); // 1+3=4
        e.set("k", b"bb".to_vec()).await.unwrap(); // 1+2=3, total 4-4+3=3
        assert_eq!(e.current_usage_bytes().await.unwrap(), 3);
        assert_eq!(e.get("k").await.unwrap().unwrap(), b"bb");
    }

    /// Interaction via BlinkStorage trait (abstraction layer).
    #[tokio::test]
    async fn trait_interface() {
        let store: std::sync::Arc<dyn BlinkStorage> =
            std::sync::Arc::new(engine(1024));
        store.set("trait_key", b"trait_value".to_vec()).await.unwrap();
        let v = store.get("trait_key").await.unwrap().unwrap();
        assert_eq!(v, b"trait_value");
        assert!(store.delete("trait_key").await.unwrap());
    }
}
