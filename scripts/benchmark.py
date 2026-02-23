#!/usr/bin/env python3
"""Benchmark Blink-Store: throughput, latency, memory cap enforcement, and LRU eviction."""

import socket
import statistics
import sys
import time

HOST = "127.0.0.1"
PORT = 8765

def connect():
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    sock.connect((HOST, PORT))
    reader = sock.makefile("r", encoding="utf-8", newline="\n")
    return sock, reader

def send(sock, reader, cmd):
    sock.sendall((cmd + "\n").encode())
    return reader.readline().strip()

def get_usage(sock, reader):
    resp = send(sock, reader, "USAGE")
    return int(resp.split()[1])

def bench_throughput(label, cmd_fn, count):
    sock, reader = connect()
    t0 = time.perf_counter()
    for i in range(count):
        cmd_fn(sock, reader, i)
    elapsed = time.perf_counter() - t0
    sock.close()
    ops = count / elapsed
    return elapsed, ops

def bench_latency(label, cmd_fn, count):
    sock, reader = connect()
    latencies = []
    for i in range(count):
        t0 = time.perf_counter()
        cmd_fn(sock, reader, i)
        latencies.append((time.perf_counter() - t0) * 1_000_000)
    sock.close()
    latencies.sort()
    return {
        "min": latencies[0],
        "p50": latencies[len(latencies) // 2],
        "p95": latencies[int(len(latencies) * 0.95)],
        "p99": latencies[int(len(latencies) * 0.99)],
        "max": latencies[-1],
        "avg": statistics.mean(latencies),
    }

def fmt_us(us):
    if us < 1000:
        return f"{us:.0f} us"
    return f"{us / 1000:.2f} ms"

def main():
    N = 10_000
    VALUE_64 = "x" * 64
    VALUE_256 = "x" * 256
    VALUE_1K = "x" * 1024

    print("=" * 62)
    print("  Blink-Store Benchmark")
    print("=" * 62)
    print(f"  Server       : {HOST}:{PORT}")
    print(f"  Operations   : {N:,} per test")
    print(f"  Connection   : single persistent TCP")
    print()

    # --- Throughput ---
    print("Throughput (sequential, single connection)")
    print("-" * 62)

    results = {}

    for label, val in [("SET 64B", VALUE_64), ("SET 256B", VALUE_256), ("SET 1KiB", VALUE_1K)]:
        elapsed, ops = bench_throughput(
            label,
            lambda s, r, i, v=val: send(s, r, f"SET bench_{i} {v}"),
            N,
        )
        results[label] = ops
        print(f"  {label:<12} : {ops:>10,.0f} ops/sec  ({elapsed:.2f}s)")

    elapsed, ops = bench_throughput(
        "GET",
        lambda s, r, i: send(s, r, f"GET bench_{i % N}"),
        N,
    )
    results["GET"] = ops
    print(f"  {'GET':<12} : {ops:>10,.0f} ops/sec  ({elapsed:.2f}s)")

    elapsed, ops = bench_throughput(
        "DELETE",
        lambda s, r, i: send(s, r, f"DELETE bench_{i}"),
        N,
    )
    results["DELETE"] = ops
    print(f"  {'DELETE':<12} : {ops:>10,.0f} ops/sec  ({elapsed:.2f}s)")

    print()

    # --- Latency ---
    print("Latency (sequential, single connection, 10K samples)")
    print("-" * 62)

    # Seed data for GET latency test
    sock, reader = connect()
    for i in range(N):
        send(sock, reader, f"SET lbench_{i} {VALUE_256}")
    sock.close()

    for label, cmd_fn in [
        ("SET 256B", lambda s, r, i: send(s, r, f"SET lbench_{i} {VALUE_256}")),
        ("GET", lambda s, r, i: send(s, r, f"GET lbench_{i}")),
    ]:
        lat = bench_latency(label, cmd_fn, N)
        print(f"  {label}:")
        print(f"    avg={fmt_us(lat['avg'])}  p50={fmt_us(lat['p50'])}  p95={fmt_us(lat['p95'])}  p99={fmt_us(lat['p99'])}  max={fmt_us(lat['max'])}")

    print()

    # --- Memory cap test ---
    LIMIT = 2_097_152
    print("Memory cap enforcement (2 MiB limit, 1 KiB values)")
    print("-" * 62)
    sock, reader = connect()

    for i in range(1, 4001):
        send(sock, reader, f"SET memtest_{i} {VALUE_1K}")
    usage = get_usage(sock, reader)
    cap_ok = usage <= LIMIT
    print(f"  Inserted 4,000 x 1 KiB keys into 2 MiB store")
    print(f"  Final usage : {usage:,} bytes ({usage / 1024:.0f} KiB / {LIMIT / 1024:.0f} KiB limit)")
    print(f"  {'PASS' if cap_ok else 'FAIL'}: usage {'<=' if cap_ok else '>'} limit")
    print()

    # --- Eviction test ---
    # Sampled eviction (like Redis): not strictly LRU, but older keys
    # are statistically more likely to be evicted.
    print("Sampled eviction")
    print("-" * 62)
    evicted = 0
    checked_early = [1, 2, 3, 10, 50, 100]
    for i in checked_early:
        resp = send(sock, reader, f"GET memtest_{i}")
        status = "evicted" if resp == "NOT_FOUND" else "present"
        if resp == "NOT_FOUND":
            evicted += 1
        print(f"  memtest_{i:<5} -> {status}")

    recent_present = 0
    for i in range(3996, 4001):
        resp = send(sock, reader, f"GET memtest_{i}")
        if resp != "NOT_FOUND":
            recent_present += 1

    eviction_ok = evicted >= len(checked_early) // 2
    print(f"  Early keys evicted  : {evicted}/{len(checked_early)}")
    print(f"  Recent keys (3996-4000): {recent_present}/5 present")
    sock.close()
    print()

    # --- Summary ---
    all_pass = cap_ok and eviction_ok and recent_present == 5
    print("=" * 62)
    print("  Results Summary")
    print("=" * 62)
    print(f"  SET 64B throughput  : {results['SET 64B']:,.0f} ops/sec")
    print(f"  SET 256B throughput : {results['SET 256B']:,.0f} ops/sec")
    print(f"  SET 1KiB throughput : {results['SET 1KiB']:,.0f} ops/sec")
    print(f"  GET throughput      : {results['GET']:,.0f} ops/sec")
    print(f"  DELETE throughput   : {results['DELETE']:,.0f} ops/sec")
    print(f"  Memory cap          : {'PASS' if cap_ok else 'FAIL'}")
    print(f"  Sampled eviction    : {'PASS' if eviction_ok else 'FAIL'} ({evicted}/{len(checked_early)} early keys evicted)")
    print()
    print(f"  OVERALL: {'ALL TESTS PASSED' if all_pass else 'SOME TESTS FAILED'}")

    sys.exit(0 if all_pass else 1)

if __name__ == "__main__":
    main()
