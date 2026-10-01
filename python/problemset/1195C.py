import sys
import collections
class Scanner:
    __slots__ = ("_it",)

    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def ints(self, n):
        return [int(next(self._it)) for _ in range(n)]

def solve(sc):
    n = sc.i()
    h1 = sc.ints(n)
    h2 = sc.ints(n)
    best1 = best2 = 0
    for i in range(n):
        best1, best2 = max(best1, best2 + h1[i]), max(best2, best1 + h2[i])
    print(max(best1, best2))




def main():
    sc = Scanner()
    solve(sc)

if __name__ == "__main__":
    main()
