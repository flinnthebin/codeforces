#!/usr/bin/env python3

import sys

data = list(map(int, sys.stdin.read().split()))
data.sort()

c = data[3] - data[0]
b = data[3] - data[1]
a = data[3] - data[2]

print(a, b, c)
