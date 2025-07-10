#!/usr/bin/env python3

import sys
lines = sys.stdin.read().splitlines()

n = int(lines[0])
drinks = list(map(int, lines[1].split()))

vol = 0
for x in range(n):
    vol += drinks[x]

res = (vol / (n * 100)) * 100

print(res)
