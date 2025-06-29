#!/usr/bin/env python3

import sys

def eat_candies(arr: list[int]):
    res = 0
    minval = min(arr)
    for val in arr:
        res += val - minval
    return res



def main():
    lines = sys.stdin.read().splitlines()

    t = int(lines[0])
    ptr = 1

    results = []

    for _ in range(t):
        n = int(lines[ptr])
        candies = list(map(int, lines[ptr + 1].split()))
        results.append(eat_candies(candies))
        ptr += 2

    for r in results:
        print(r)

main()
