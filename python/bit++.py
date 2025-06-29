#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()

n = int(lines[0])
x = int(0)
ptr = 1

for _ in range(n):
    val = lines[ptr]
    if '+' in val:
        x += 1
    if '-' in val:
        x -=1
    ptr += 1

print(x)

