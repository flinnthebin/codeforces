#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()
n, k = map(int, lines[0].split())
scores = list(map(int, lines[1].split()))
limit = scores[k-1]

count = sum(1 for s in scores if s >= limit and s > 0)
print(count)
