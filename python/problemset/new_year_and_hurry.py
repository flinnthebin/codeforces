#!/usr/bin/env python3

def solvable(x, time_limit):
    return 5 * x * (x + 1) // 2 <= time_limit

n, k = map(int, input().split())
time_left = 240 - k

low, high = 0, n
res = 0

while low <= high:
    mid = (low + high) // 2
    if solvable(mid, time_left):
        res = mid
        low = mid + 1
    else:
        high = mid - 1

print(res)
