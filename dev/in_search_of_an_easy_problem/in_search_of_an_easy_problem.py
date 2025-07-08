#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()
n = int(lines[0])
ans = list(map(int, lines[1].split()))

print("HARD" if 1 in ans else "EASY")
