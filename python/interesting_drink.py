#!/usr/bin/env python3

import sys
import bisect

lines = sys.stdin.read().splitlines()

n = int(lines[0])
prices = sorted((map(int, lines[1].split())))
q = int(lines[2])
coins = [int(line) for line in lines[3:]]

for m in coins:
    count = bisect.bisect_right(prices, m)
    print(count)
