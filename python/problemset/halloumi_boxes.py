#!/usr/bin/env/python3
import sys

data = iter(sys.stdin.read().split())

t = int(next(data))
for _ in range(t):
    n = int(next(data))
    k = int(next(data))
    arr = []
    for _ in range(n):
        arr.append(int(next(data)))
    if arr == sorted(arr) or k > 1:
        print("YES")
    else:
        print("NO")
