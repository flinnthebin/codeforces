#!/usr/bin/env python3

import sys
lines = sys.stdin.read().splitlines()

n = int(lines[0])
arr = list(map(int, lines[1].split()))
res = []
for i in range(n + 1):
    if i in arr:
        res.append(arr.index(i))
for j in range(n):
    res[j] += 1

strres = ' '.join([str(n) for n in res])
print(strres)
