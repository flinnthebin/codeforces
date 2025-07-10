#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()
n, t = map(int, lines[0].split())
q = list(lines[1])

for _ in range(t):
    x = 0
    while x < n - 1:
        if q[x] == 'B' and q[x+1] == 'G':
            q[x], q[x+1] = q[x+1], q[x]
            x += 1
        x += 1
print(''.join(q))
