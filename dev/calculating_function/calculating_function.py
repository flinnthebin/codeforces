#!/usr/bin/env python3

n = int(input())

if n % 2 == 0:
    print(n // 2)
else:
    print(-(n // 2 + 1))
