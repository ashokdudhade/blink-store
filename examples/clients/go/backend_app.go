// Minimal HTTP backend using Blink Store as cache. GET /key, POST /key with body.
// Start store: cargo run -- serve --tcp 127.0.0.1:8765
// Run: BLINK_STORE=127.0.0.1:8765 go run backend_app.go

package main

import (
	"bufio"
	"encoding/base64"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"strings"
)

func storeAddr() string {
	if a := os.Getenv("BLINK_STORE"); a != "" {
		return a
	}
	return "127.0.0.1:8765"
}

func storeRequest(cmd, key, value string) (string, error) {
	addr := storeAddr()
	conn, err := net.Dial("tcp", addr)
	if err != nil {
		return "", err
	}
	defer conn.Close()
	req := cmd + " " + key
	if value != "" {
		req += " " + value
	}
	req += "\n"
	if _, err := conn.Write([]byte(req)); err != nil {
		return "", err
	}
	line, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		return "", err
	}
	return strings.TrimSuffix(line, "\n"), nil
}

func main() {
	port := "8080"
	if p := os.Getenv("PORT"); p != "" {
		port = p
	}
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		key := strings.Trim(r.URL.Path, "/")
		if key == "" {
			http.Error(w, "Use /<key>", http.StatusBadRequest)
			return
		}
		switch r.Method {
		case http.MethodGet:
			resp, err := storeRequest("GET", key, "")
			if err != nil {
				http.Error(w, "Bad Gateway", http.StatusBadGateway)
				return
			}
			if resp == "NOT_FOUND" {
				http.Error(w, "Not Found", http.StatusNotFound)
				return
			}
			if strings.HasPrefix(resp, "VALUE ") {
				b64 := strings.TrimSpace(resp[6:])
				dec, err := base64.StdEncoding.DecodeString(b64)
				if err != nil {
					http.Error(w, "Bad Gateway", http.StatusBadGateway)
					return
				}
				w.Header().Set("Content-Type", "application/octet-stream")
				w.Write(dec)
				return
			}
			http.Error(w, "Bad Gateway", http.StatusBadGateway)
		case http.MethodPost:
			body, _ := io.ReadAll(r.Body)
			bodyStr := string(body)
			resp, err := storeRequest("SET", key, bodyStr)
			if err != nil || !strings.HasPrefix(resp, "OK") {
				http.Error(w, "Bad Gateway", http.StatusBadGateway)
				return
			}
			w.WriteHeader(http.StatusNoContent)
		default:
			http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		}
	})
	fmt.Printf("Backend on http://0.0.0.0:%s (store: %s)\n", port, storeAddr())
	http.ListenAndServe(":"+port, nil)
}
