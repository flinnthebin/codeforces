#!/usr/bin/env python3

print((lambda k, n, w: max(k*w*(w+1)//2 - n, 0))(*map(int, input().split())))
