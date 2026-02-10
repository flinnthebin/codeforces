#!/usr/bin/env python3

import sys

data = iter(sys.stdin.read().split())
t = int(next(data))

for _ in range(t):
    n = int(next(data))
    x = int(next(data))
    y = int(next(data))
    total = 0
    max = 0
    for _ in range(n):
        a_i = int(next(data))
        transferable = (a_i // x) * y
        remainder = a_i - transferable
        total += transferable
        if remainder > max:
            max = remainder
    print(total + max)
