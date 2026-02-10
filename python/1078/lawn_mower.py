#!/usr/bin/env python3

import sys

input = iter(sys.stdin.read().split())
t = int(next(input))

for _ in range(t):
    n = int(next(input))
    w = int(next(input))

    ans = n - (n // w)
    print(ans)
