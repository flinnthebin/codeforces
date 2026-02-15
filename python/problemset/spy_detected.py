#!/usr/bin/env python3
import sys

data = iter(sys.stdin.read().split())
t = int(next(data))

for _ in range(t):
    n = int(next(data))
    arr = []
    for _ in range(n):
        arr.append(int(next(data)))
    if arr[0] == arr[1]:
        eq = arr[0]
    elif arr[0] == arr[2]:
        eq = arr[0]
    else:
        eq = arr[1]
    for i in range(n):
        if arr[i] != eq:
            print(i + 1)
            break
