#!/usr/bin/env python3

import sys

def elimination(n: int, j: int, k: int, players: list[int]) -> bool:
    player_strength = players[j - 1]
    stronger = sum(i >= player_strength for i in players)
    return stronger >= n - k

lines = sys.stdin.read().splitlines()

t = int(lines[0])
idx = 1
for _ in range(t):
    first = lines[idx]
    n, j, k = map(int, first.split())
    second = list(map(int, lines[idx + 1].split()))
    print("YES" if elimination(n, j, k, second) else "NO")
    idx += 2
