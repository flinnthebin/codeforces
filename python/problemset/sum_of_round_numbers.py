#!/usr/bin/env python3
import sys

def is_round(n: int) -> list:
    s = str(n)
    z = s[::-1]
    c = 1
    res = []
    for x in range(len(z)):
        if z[x] == "0":
            pass
        else:
            res.append(int(z[x]) * c)
        c = c * 10
    return res

data = iter(sys.stdin.read().split())
t = int(next(data))

for _ in range(t):
    n = int(next(data))
    res = is_round(n)
    print(len(res))
    print(*(res))
