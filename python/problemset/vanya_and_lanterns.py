#!/usr/bin/env python3

# l = length of street
# n = num lanterns

# coord system 0 - l, the ith lantern is at point ai

# d = distance illuminated by lantern, common for all

# find min(d)

import sys

def light_radius(l, lanterns):
    if len(lanterns) == 1:
        return max(lanterns[0], l - lanterns[0])
    gaps = [lanterns[i] - lanterns[i-1] for i in range(1, len(lanterns))]
    max_gap = max(gaps) / 2
    left = lanterns[0] - 0
    right = l - lanterns[-1]
    return max(left, right, max_gap) 

lines = sys.stdin.read().splitlines()

n, l = map(int, lines[0].split())
lanterns = sorted(list(map(int, lines[1].split())))

print(light_radius(l, lanterns))
