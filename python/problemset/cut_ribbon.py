#!/usr/bin/env python3
import sys
def cut_ribbon():
    n, a, b, c = map(int, sys.stdin.read().split())
    dp = [-INF] * (n + 1)
    dp[0] = 0
    for i in range(1, n + 1):
        for piece in [a, b, c]:
            if i >= piece:
                dp[i] = max(dp[i], dp[i - piece] + 1)
    print(dp[n])

cut_ribbon()
