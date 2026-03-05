#!/usr/bin/env python3

import sys

data = sys.stdin.read().split()
n = int(data[0])
p = int(data[1])
x_levels = data[2 : 2 + p]
q_idx = 2 + p
q = int(data[q_idx])
y_levels = data[q_idx + 1 : q_idx + 1 + q]
combined = set(map(int, x_levels + y_levels))

print("I become the guy." if len(combined) == n else "Oh, my keyboard!")
