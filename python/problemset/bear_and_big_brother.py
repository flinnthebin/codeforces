#!/usr/bin/env python3
from itertools import count

a, b = map(int, input().split())
print(next(i for i in count() if a * 3**i > b * 2**i))
