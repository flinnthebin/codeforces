#!/usr/bin/env python3

import sys

m, n = map(int, sys.stdin.read().split())

dominoes = (m * n) // 2

print(dominoes)
