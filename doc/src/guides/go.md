# Go

Connect to Blink-Store from Go using the standard library — no external modules needed.

---

## Prerequisites

- Go 1.18+
- Blink-Store server running ([Installation](../installation.md))

---

## Interactive client

A complete REPL client. Save as `client.go` and run it.

```go
package main

import (
	"bufio"
	"encoding/base64"
	"fmt"
	"net"
	"os"
	"strings"
)

func main() {
	host := "127.0.0.1"
	port := "8765"
	if len(os.Args) > 1 { host = os.Args[1] }
	if len(os.Args) > 2 { port = os.Args[2] }

	addr := net.JoinHostPort(host, port)
	conn, err := net.Dial("tcp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Cannot connect to %s: %v\n", addr, err)
		os.Exit(1)
	}
	defer conn.Close()

	fmt.Printf("Connected to %s\n", addr)
	fmt.Println("Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT")
	fmt.Println()

	reader := bufio.NewReader(conn)
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print("> ")

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			fmt.Print("> ")
			continue
		}
		if strings.EqualFold(line, "QUIT") {
			break
		}

		fmt.Fprintf(conn, "%s\n", line)
		resp, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Read error: %v\n", err)
			break
		}
		resp = strings.TrimSpace(resp)

		if strings.HasPrefix(resp, "VALUE ") {
			decoded, err := base64.StdEncoding.DecodeString(resp[6:])
			if err == nil {
				fmt.Println(string(decoded))
			} else {
				fmt.Println(resp)
			}
		} else {
			fmt.Println(resp)
		}
		fmt.Print("> ")
	}
}
```

**Run:**

```bash
go run client.go
```

```text
Connected to 127.0.0.1:8765
Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT

> SET lang go
OK
> GET lang
go
> QUIT
```

---

## One-off commands

A reusable function for sending a single command:

```go
func blink(command, addr string) (string, error) {
	conn, err := net.Dial("tcp", addr)
	if err != nil { return "", err }
	defer conn.Close()

	fmt.Fprintf(conn, "%s\n", command)
	resp, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil { return "", err }
	resp = strings.TrimSpace(resp)

	if strings.HasPrefix(resp, "VALUE ") {
		decoded, err := base64.StdEncoding.DecodeString(resp[6:])
		if err != nil { return resp, nil }
		return string(decoded), nil
	}
	return resp, nil
}
```

---

## Key concepts

| Concept | Go API |
|---------|--------|
| TCP connection | `net.Dial("tcp", addr)` |
| Line reading | `bufio.NewReader(conn).ReadString('\n')` |
| Base64 decode | `base64.StdEncoding.DecodeString(payload)` |
| Error handling | Check `err` on every call |
