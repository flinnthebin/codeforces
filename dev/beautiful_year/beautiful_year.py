#!/usr/bin/env python3

x = int(input())

while True:
    x += 1
    if len(set(str(x))) == len(str(x)):
        break
print(x)
