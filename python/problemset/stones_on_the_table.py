#!/usr/bin env python3

n, s = __import__('sys').stdin.read().splitlines()
print(sum(a == b for a, b in zip(s, s[1:])))

