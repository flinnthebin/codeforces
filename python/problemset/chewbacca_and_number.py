#!/usr/bin/env python3
import sys

x = sys.stdin.read().split()[0]
lsx = [int(digit) for digit in str(x)]

for i in range(len(lsx)):
    if i == 0 and lsx[i] == 9:
        continue
    else:
        lsx[i] = min(lsx[i], 9 - lsx[i])
print(int("".join(map(str,lsx))))
