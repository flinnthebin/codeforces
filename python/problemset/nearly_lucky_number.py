#!/usr/bin/env python3

num = input()
cnt = sum(1 for c in num if c in '47')
print("YES" if cnt and all(c in '47' for c in str(cnt)) else "NO")
