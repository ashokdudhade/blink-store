# Contributing to Blink Store

## Adding a new language client

1. **Implement the protocol**  
   See [docs/PROTOCOL_SPEC.md](docs/PROTOCOL_SPEC.md) and [docs/protocol.md](protocol.md). Commands: `GET`, `SET`, `DELETE`, `USAGE`, `QUIT`. Responses: `OK`, `VALUE <base64>`, `NOT_FOUND`, `USAGE <n>`, `ERROR <msg>`.

2. **Place the client**  
   - REPL client: `examples/clients/<language>/blink_client.<ext>`  
   - Optional HTTP backend: `examples/clients/<language>/backend_app.<ext>`

3. **Quickstart**  
   Add a short Quickstart in `examples/clients/<language>/README.md` (or one line in the main [examples/clients/README.md](examples/clients/README.md)): how to run the client against a server on `127.0.0.1:8765`.

4. **Test**  
   Run `./scripts/test_examples.sh` and extend it if needed so the new client is exercised (e.g. REPL SET/GET/QUIT).

## Code style

Follow the project [.cursor/rules.md](.cursor/rules.md): no `unwrap`/`expect` in library code, use `tracing` for logs, `thiserror`/`anyhow` for errors.

## Docs

- Protocol: `docs/PROTOCOL_SPEC.md`, `docs/protocol.md`  
- Integration: `docs/INTEGRATION_GUIDES.md`  
- Deployment: `docs/DEPLOYMENT.md`
