#!/usr/bin/env python3

import sys
lines = sys.stdin.read().splitlines()
n = int(lines[0])
magnets = list(map(int, lines[1:]))

prev = 0
count = 0
for m in range(n):
    if prev != magnets[m]
        count += 1
    prev = magnets[m]

print(count)
