#!/usr/bin/env python3

def is_almost_lucky(n):
    lucky_numbers = [4, 7, 44, 47, 74, 77]

    for lucky in lucky_numbers:
        if n % lucky == 0:
            return "YES"

    return "NO"

n = list(input())
flag = True
for i in n:
    if i not in '47':
        flag = False
x = int(''.join(n))

print("YES" if flag == True else is_almost_lucky(x))
