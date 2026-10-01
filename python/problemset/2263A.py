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

    def tok(self):
        return next(self._it)

    def i(self):
        return int(next(self._it))

    def ints(self, n):
        return [int(next(self._it)) for _ in range(n)]

def solve(sc, out):
    n = sc.i();
    arr = sc.ints(n)
    a = arr.count(1)
    b = arr.count(0)
    result = "Bessie" if a >= b else "Elsie"
    out.append(result)

def main():
    sc = Scanner()
    out = []
    for _ in range(sc.i()):
        solve(sc, out)
    sys.stdout.write("\n".join(out) + "\n")

if __name__ == "__main__":
    main()
