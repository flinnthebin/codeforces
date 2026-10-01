import sys
from bisect import bisect_left, bisect_right, insort
from collections import Counter, defaultdict, deque
from heapq import heapify, heappop, heappush
from itertools import accumulate, combinations, permutations
from math import comb, gcd, isqrt, lcm, perm

class Scanner:
    __slots__ = ("_it",)

    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def ints(self, n):
        [int(next(self._it)) for _ in range(n)]

def main():
    sc = Scanner()
    c, b = 0, 0
    for _ in range(sc.i()):
        e = sc.i();
        if e == -1:
            if b > 0:
                b -= 1
            else:
                c += 1
        else:
            b += e
    print(0 if c <= 0 else c)

if __name__ == "__main__":
    main()


