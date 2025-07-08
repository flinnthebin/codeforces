#!/usr/bin/env python3
import sys

lines = sys.stdin.read().splitlines()
n = int(lines[0])
vectors = [list(map(int, line.split())) for line in lines[1:]]

xcount = ycount = zcount = 0

for x, y, z in vectors:
    xcount += x
    ycount += y
    zcount += z

if xcount == ycount == zcount == 0:
    print("YES")
else:
    print("NO")

