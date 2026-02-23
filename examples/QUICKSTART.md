# Quickstart by language

No Git clone required. Install the **latest** server and start it:

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh | bash -s -- latest ./bin
./bin/blink-store serve --tcp 127.0.0.1 8765
```

Then in another terminal, run a client (download the script and run it):

| Language | One-liner (no clone) |
|----------|----------------------|
| **Python** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/python/blink_client.py -o blink_client.py && python3 blink_client.py 127.0.0.1 8765` |
| **Node** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/node/blink_client.js -o blink_client.js && node blink_client.js 127.0.0.1 8765` |
| **Go** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/go/blink_client.go -o blink_client.go && go run blink_client.go 127.0.0.1 8765` |
| **Shell** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/shell/blink_client.sh -o blink_client.sh && bash blink_client.sh 127.0.0.1 8765` |

Try: `SET hello world`, `GET hello`, `USAGE`, `QUIT`.

If you have the repo cloned, you can use `./scripts/install-from-github.sh latest ./bin` and `python examples/clients/python/blink_client.py` etc. instead.
