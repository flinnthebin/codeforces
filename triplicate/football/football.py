#!/usr/bin/env python3

num = input()

zeroes=0
ones=0
res=False
for c in num:
    if int(c) == 0:
        zeroes += 1
        ones = 0
    else:
        ones += 1
        zeroes = 0
    if zeroes >= 7 or ones >= 7:
        res=True

if res:
    print("YES")
else:
    print("NO")
