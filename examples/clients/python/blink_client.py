#!/usr/bin/env python3
"""Example Blink-Store client (TCP). GET/SET/DELETE/USAGE/QUIT, VALUE is base64.
Usage: python blink_client.py [host [port]]"""

import base64
import socket
import sys

def main():
    host = sys.argv[1] if len(sys.argv) > 1 else "127.0.0.1"
    port = int(sys.argv[2]) if len(sys.argv) > 2 else 8765
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((host, port))
        reader = sock.makefile("r", encoding="utf-8", newline="\n")
        writer = sock.makefile("w", encoding="utf-8", newline="\n")
        print("Commands: GET <key> | SET <key> <value> | DELETE <key> | USAGE | QUIT", flush=True)
        try:
            while True:
                line = input().strip()
                if not line or line.upper() == "QUIT":
                    break
                writer.write(line + "\n")
                writer.flush()
                resp = reader.readline()
                if not resp:
                    break
                resp = resp.rstrip("\n")
                if resp.startswith("VALUE "):
                    try:
                        out = base64.b64decode(resp[6:].strip()).decode("utf-8", errors="replace")
                        print(out)
                    except Exception:
                        print(resp)
                else:
                    print(resp)
        finally:
            try:
                writer.write("QUIT\n")
                writer.flush()
            except Exception:
                pass

if __name__ == "__main__":
    main()
