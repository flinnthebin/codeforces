#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()

t = int(lines[0])
tickets = lines[1:t+1]

results = [
    "YES" if sum(map(int, ticket[:3])) == sum(map(int, ticket[3:])) else "NO"
        for ticket in tickets
]

print('\n'.join(results))
