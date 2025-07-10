#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()
n = int(lines[0])
rooms = [line.split() for line in lines[1:]]

print(sum(1 if int(rooms[i][1]) - int(rooms[i][0]) >= 2 else 0 for i in range(n)))
