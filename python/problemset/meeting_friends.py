#!/usr/bin/env python3

import sys

data = iter(sys.stdin.read().split())

a = int(next(data))
b = int(next(data))
c = int(next(data))

coords = [a, b, c]
coords.sort()

print((coords[2] - coords[1]) + (coords[1] - coords[0]))
