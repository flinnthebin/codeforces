#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()

t = int(lines[0])

for x in range(1, t + 1):
    n, k = map(int, lines[x].split())
    print(k + (k - 1) // (n - 1))
