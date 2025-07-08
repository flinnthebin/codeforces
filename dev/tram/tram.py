#!/usr/bin/env python3

import sys
lines = sys.stdin.read().splitlines()

n = int(lines[0])

stops = [list(map(int, line.split())) for line in lines[1:]]

count = 0
max = 0
for x in range(len(stops)):
    count -= stops[x][0]
    count += stops[x][1]
    if count > max:
        max = count

print(max)
