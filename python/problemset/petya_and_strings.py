#!/usr/bin/env python3

import sys

first, second = sys.stdin.read().splitlines()

if first.lower() == second.lower():
    print("0")
elif first.lower() < second.lower():
    print("-1")
else:
    print("1")
