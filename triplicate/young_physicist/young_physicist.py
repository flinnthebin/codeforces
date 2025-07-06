#!/usr/bin/env python3
import sys

n = int(input())
vectors = list(map(int, sys.stdin.read().split()))

x = 0
y = 1
z = 2

xcount = 0
ycount = 0
zcount = 0

for i in range(n):
    xcount += vectors[x]
    ycount += vectors[y]
    zcount += vectors[z]

    x += 3
    y += 3
    z += 3

if xcount == 0 and ycount == 0 and zcount == 0:
    print("YES")
else:
    print("NO")
