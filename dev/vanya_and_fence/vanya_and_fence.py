#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()

n, h = map(int, lines[0].split())
friends = list(map(int, lines[1].split()))

count = sum(1 if c <= h else 2 for c in friends)

print(count)
