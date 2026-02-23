# Protocol

Line-based text over TCP or Unix socket. Commands: GET key, SET key value, DELETE key, USAGE, QUIT. Responses: OK, VALUE base64, NOT_FOUND, USAGE n, ERROR msg. See docs/PROTOCOL_SPEC.md in the repo.
