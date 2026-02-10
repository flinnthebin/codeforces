#!/usr/bin/env python3
import sys
from collections import deque

lines = sys.stdin.read().splitlines()

n = int(lines[0])
cards = deque(map(int, lines[1].split()))
sereja = dima = c = 0
while cards:
    if cards[0] > cards[-1]:
        chosen = cards[0]
        cards.popleft()
    else:
        chosen = cards[-1]
        cards.pop()
    if c % 2 == 0:
        sereja += chosen
    else:
        dima += chosen
    c += 1

print(sereja, dima)
