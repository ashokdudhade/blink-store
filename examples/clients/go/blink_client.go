// Example Blink-Store client (TCP). Protocol: GET/SET/DELETE/USAGE/QUIT, VALUE is base64.
//
// Usage:
//
//	go run blink_client.go [host [port]]
//	go run blink_client.go 127.0.0.1 8765
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
	if len(os.Args) > 1 {
		host = os.Args[1]
	}
	if len(os.Args) > 2 {
		port = os.Args[2]
	}
	addr := net.JoinHostPort(host, port)

	conn, err := net.Dial("tcp", addr)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	defer conn.Close()

	reader := bufio.NewReader(conn)
	scanner := bufio.NewScanner(os.Stdin)

	fmt.Println("Commands: GET <key> | SET <key> <value> | DELETE <key> | USAGE | QUIT")

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}
		if strings.ToUpper(line) == "QUIT" {
			break
		}

		if _, err := fmt.Fprintf(conn, "%s\n", line); err != nil {
			fmt.Fprintln(os.Stderr, err)
			break
		}

		resp, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		resp = strings.TrimSuffix(resp, "\n")

		if strings.HasPrefix(resp, "VALUE ") {
			b64 := strings.TrimSpace(resp[6:])
			dec, err := base64.StdEncoding.DecodeString(b64)
			if err != nil {
				fmt.Println(resp)
			} else {
				fmt.Println(string(dec))
			}
		} else {
			fmt.Println(resp)
		}
	}
}
