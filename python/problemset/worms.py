#!/usr/bin/env python3

import sys
import bisect as b

lines = sys.stdin.read().splitlines()

n = int(lines[0])
worms = list(map(int, lines[1].split()))
m = int(lines[2])
labels = list(map(int, lines[3].split()))

prefix = []
total = 0
for w in worms:
    total += w
    prefix.append(total)

for l in labels:
    pile = b.bisect_left(prefix, l) + 1
    print(pile)
