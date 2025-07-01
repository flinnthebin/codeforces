#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()

t = int(lines[0])
idx = 1
for _ in range(t):
    first = lines[idx]
    second = lines[idx + 1]
    idx += 2

