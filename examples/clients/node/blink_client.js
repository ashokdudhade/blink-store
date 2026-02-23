#!/usr/bin/env node
/** Example Blink Store client (TCP). Protocol: GET/SET/DELETE/USAGE/QUIT, VALUE is base64.
 *
 * Usage:
 *   node blink_client.js [host [port]]
 *   node blink_client.js 127.0.0.1 8765
 */

const net = require('net');
const readline = require('readline');

const host = process.argv[2] || '127.0.0.1';
const port = parseInt(process.argv[3], 10) || 8765;

const sock = net.createConnection(port, host, () => {});
const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

let buffer = '';

sock.on('data', (data) => {
  buffer += data.toString();
  let idx;
  while ((idx = buffer.indexOf('\n')) !== -1) {
    const line = buffer.slice(0, idx).trim();
    buffer = buffer.slice(idx + 1);
    if (line.startsWith('VALUE ')) {
      const b64 = line.slice(6).trim();
      try {
        console.log(Buffer.from(b64, 'base64').toString('utf8'));
      } catch {
        console.log(line);
      }
    } else {
      console.log(line);
    }
  }
});

sock.on('close', () => process.exit(0));
sock.on('error', (err) => { console.error(err); process.exit(1); });

console.log('Commands: GET <key> | SET <key> <value> | DELETE <key> | USAGE | QUIT');

rl.on('line', (line) => {
  line = line.trim();
  if (!line) return;
  if (line.toUpperCase() === 'QUIT') {
    sock.write('QUIT\n');
    sock.end();
    return;
  }
  sock.write(line + '\n');
});
