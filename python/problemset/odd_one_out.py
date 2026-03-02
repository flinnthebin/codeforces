#!/usr/bin/env python3

import sys

data = iter(sys.stdin.read().split())
t = int(next(data))

for _ in range(t):
    a = int(next(data))
    b = int(next(data))
    c = int(next(data))

    if a == b:
        print(c)
    elif a == c:
        print(b)
    else:
        print(a)

