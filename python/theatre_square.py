#!/usr/bin/env python3

n, m, a = map(int, input().split())

flagstones_n = (n + a - 1) // a
flagstones_m = (m + a - 1) // a

total = flagstones_n * flagstones_m

print(total)
