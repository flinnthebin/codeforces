#!/usr/bin/env python3
print((lambda n, k:[n := n // 10 if n % 10 == 0 else n - 1 for _ in range(k)][-1])(*map(int, input().split())))
