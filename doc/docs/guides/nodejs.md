---
title: Node.js
---

# Node.js

Connect to Blink Store from Node.js using the built-in `net` module — no npm packages needed.

---

## Prerequisites

- Node.js 14+
- Blink Store server running ([Installation](../installation))

---

## Interactive client

A complete REPL client. Save as `client.js` and run it.

```javascript
const net = require('net');
const readline = require('readline');

const host = process.argv[2] || '127.0.0.1';
const port = parseInt(process.argv[3], 10) || 8765;

const sock = net.createConnection(port, host, () => {
  console.log(`Connected to ${host}:${port}`);
  console.log('Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT\n');
  rl.prompt();
});

const rl = readline.createInterface({ input: process.stdin, prompt: '> ' });
let buffer = '';

sock.on('data', (data) => {
  buffer += data.toString();
  let idx;
  while ((idx = buffer.indexOf('\n')) !== -1) {
    const line = buffer.slice(0, idx).trim();
    buffer = buffer.slice(idx + 1);
    if (line.startsWith('VALUE ')) {
      console.log(Buffer.from(line.slice(6).trim(), 'base64').toString('utf8'));
    } else {
      console.log(line);
    }
    rl.prompt();
  }
});

rl.on('line', (input) => {
  const line = input.trim();
  if (!line) { rl.prompt(); return; }
  if (line.toUpperCase() === 'QUIT') {
    sock.write('QUIT\n');
    sock.end();
    return;
  }
  sock.write(line + '\n');
});

sock.on('close', () => process.exit(0));
sock.on('error', (err) => { console.error('Error:', err.message); process.exit(1); });
```

**Run:**

```bash
node client.js
```

```text
Connected to 127.0.0.1:8765
Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT

> SET session abc123
OK
> GET session
abc123
> QUIT
```

---

## One-off commands

Send a single command and get the result as a Promise:

```javascript
const net = require('net');

function blink(command, host = '127.0.0.1', port = 8765) {
  return new Promise((resolve, reject) => {
    const sock = net.createConnection(port, host, () => {
      sock.write(command + '\n');
    });
    let data = '';
    sock.on('data', (chunk) => { data += chunk.toString(); });
    sock.on('end', () => {
      const resp = data.trim();
      if (resp.startsWith('VALUE '))
        resolve(Buffer.from(resp.slice(6), 'base64').toString());
      else
        resolve(resp);
    });
    sock.on('error', reject);
  });
}

// Usage
(async () => {
  await blink('SET token xyz');
  console.log(await blink('GET token')); // -> "xyz"
  console.log(await blink('USAGE'));     // -> "USAGE 8"
})();
```

---

## Key concepts

| Concept | Node.js API |
|---------|------------|
| TCP connection | `net.createConnection(port, host)` |
| Line reading | Buffer incoming data, split on `\n` |
| Base64 decode | `Buffer.from(payload, 'base64').toString()` |
| Async pattern | Wrap socket in a `Promise` |
