#!/usr/bin/env python3

import sys

def winner(x: int):
    nums = [y for y in range(x)]
    zeroes = [n % 4 for n in nums].count(0)
    ones = [n % 4 for n in nums].count(1)
    twos = [n % 4 for n in nums].count(2)
    threes = [n % 4 for n in nums].count(3)

    rounds = min(zeroes, threes) + min(ones, twos)

    if x - 2*rounds:
        return "Alice"
    else:
        return "Bob"


lines = sys.stdin.read().splitlines()

t = int(lines[0])

for x in range(1, t + 1):
    print(winner(int(lines[x])))
