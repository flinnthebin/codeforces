#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()
t = int(lines[0])

for i in range(1, t + 1):
    n = int(lines[i])
    cows = n // 4
    while cows >= 0:
        if (n - 4 * cows) % 2 == 0:
            chickens = (n - 4 * cows) // 2
            print(cows + chickens)
            break
        cows -= 1

