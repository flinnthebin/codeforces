#!/usr/bin/env python3

import sys

data = iter(sys.stdin.read().split())
t = int(next(data))

for _ in range(t):
    n = int(next(data))
    if n & 1:
        print(n // 2)
    else:
        print((n // 2) - 1)
