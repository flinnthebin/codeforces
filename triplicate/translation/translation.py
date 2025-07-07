#!/usr/bin/env python3

import sys
lines = sys.stdin.read().splitlines()
print("YES" if lines[0][::-1] == lines[1] else "NO")
