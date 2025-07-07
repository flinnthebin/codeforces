#!/usr/bin/env python3

n, m, a, b = map(int, input().split())

# single-ride only
cost_a = n * a

# multi-ride w/ single leftover
full_tickets = n // m
leftover = n % m
cost_b = full_tickets * b + leftover * a

# multi-ride only
cost_c = ((n + m - 1) // m) * b

print(min(cost_a, cost_b, cost_c))
