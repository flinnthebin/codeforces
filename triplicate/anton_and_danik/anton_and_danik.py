#!/usr/bin/env python3

import sys

s = sys.stdin.read().split()[1]

print("Friendship" if s.count('A') == s.count('D') else "Anton" if s.count('A') > s.count('D') else "Danik")
