#!/usr/bin/env python3

import sys
import bisect

lines = sys.stdin.read().splitlines()

n, t = map(int, lines[0].split())
mins = sorted(list(map(int, lines[1].split())))

prefix = []
total = 0
for m in mins:
    total += m
    prefix.append(total)

if t <= mins[0]:
    res = 0
else:
    res = bisect.bisect_left(prefix, t)
print(res)
