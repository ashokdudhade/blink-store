# Rust Best Practices & Standards

## Core Principles
- **Safety First**: Strictly avoid `unsafe` code unless absolutely necessary for high-performance FFIs.
- **Zero-Cost Abstractions**: Use Generics and Traits to provide flexibility without runtime overhead.
- **Panic-Free**: Never use `.unwrap()` or `.expect()` in production code. Always handle errors using `Result<T, E>` and `?`.

## Project-Specific: Blink-Store
- **Memory Tracking**: All storage operations must track size in bytes. Use `std::mem::size_of_val` or manual counting for `Vec<u8>`.
- **Async Pattern**: Prefer `tokio` for the runtime. Use `tokio::sync` for synchronization rather than `std::sync` in async contexts to avoid blocking the executor.
- **Abstraction**: Code should interact with the `BlinkStorage` trait, not the concrete `MemoryEngine`.

## Coding Style
- **Naming**: Use `snake_case` for functions/variables, `PascalCase` for Types/Traits, and `SCREAMING_SNAKE_CASE` for constants.
- **Error Handling**: Use the `thiserror` crate for library-level errors and `anyhow` for CLI/Application level errors.
- **Dependencies**: Keep the dependency tree lean. Prefer small, modular crates (e.g., `lru` over building from scratch).

## Performance Optimization
- **Allocations**: Avoid unnecessary `.clone()`. Use references `&str` or `&[u8]` where ownership isn't required.
- **Capacity**: Always use `with_capacity()` when initializing `Vec` or `HashMap` if the size is known.

## Testing & Logs
- **Test-Driven**: Create unit tests for every logic change in `src/engine.rs`.
- **Structured Logging**: Use `tracing` for all logs. Include `key` and `action` in log metadata (e.g., `info!(key = ?k, "Value set")`).