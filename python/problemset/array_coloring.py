#!/usr/bin/env python3
import sys
data = iter(sys.stdin.read().split())
t = int(next(data))
for _ in range(t):
    n = int(next(data))
    l = []
    for _ in range(n):
        l.append(int(next(data)))
    print("YES" if (sum(l) & 1) == 0 else "NO")
