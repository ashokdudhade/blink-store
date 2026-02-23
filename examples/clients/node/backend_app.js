// HTTP backend using Blink Store. GET /:key, POST /:key body. No deps.
// Start store: cargo run -- serve --tcp 127.0.0.1:8765
// Run: BLINK_STORE=127.0.0.1:8765 node backend_app.js

const http = require('http');
const net = require('net');

const storeAddr = process.env.BLINK_STORE || '127.0.0.1:8765';
const [host, port] = storeAddr.includes(':') ? storeAddr.split(':') : ['127.0.0.1', '8765'];

function storeRequest(cmd, key, value) {
  return new Promise((resolve, reject) => {
    const sock = net.createConnection(Number(port), host, () => {});
    let data = '';
    sock.on('data', (c) => { data += c.toString(); });
    sock.on('end', () => resolve(data.trim()));
    sock.on('error', reject);
    const line = value !== undefined ? `SET ${key} ${value}\n` : `${cmd} ${key}\n`;
    sock.write(line);
    sock.end();
  });
}

function readBody(req) {
  return new Promise((resolve) => {
    const chunks = [];
    req.on('data', (c) => chunks.push(c));
    req.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
  });
}

const server = http.createServer(async (req, res) => {
  const key = (req.url || '/').replace(/^\/+|\/+$/g, '') || null;
  if (!key) {
    res.writeHead(400, { 'Content-Type': 'text/plain' });
    res.end('Use /<key>');
    return;
  }
  if (req.method === 'GET') {
    try {
      const resp = await storeRequest('GET', key);
      if (resp === 'NOT_FOUND') { res.writeHead(404); res.end('Not Found'); return; }
      if (resp.startsWith('VALUE ')) {
        const body = Buffer.from(resp.slice(6).trim(), 'base64');
        res.writeHead(200, { 'Content-Type': 'application/octet-stream' });
        res.end(body);
        return;
      }
    } catch (e) {}
    res.writeHead(502); res.end('Bad Gateway');
    return;
  }
  if (req.method === 'POST') {
    const body = await readBody(req);
    try {
      const resp = await storeRequest('SET', key, body);
      if (resp.startsWith('OK')) { res.writeHead(204); res.end(); return; }
    } catch (e) {}
    res.writeHead(502); res.end('Bad Gateway');
    return;
  }
  res.writeHead(405); res.end('Method Not Allowed');
});

const p = Number(process.env.PORT) || 8080;
server.listen(p, () => console.log('Backend http://0.0.0.0:' + p + ' store=' + storeAddr));
