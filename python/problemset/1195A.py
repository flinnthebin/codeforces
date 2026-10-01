import sys
import collections
class Scanner():
    __slots__ = ("_it",)
    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def ints(self,n):
        return [int(next(self._it)) for _ in range(n)]

    def f(self):
        return float(next(self._it))

    def floats(self,n):
        return [float(next(self._it)) for _ in range(n)]

    def s(self):
        return next(self._it).decode()

def solve(sc):
    n = sc.i()
    k = sc.i()
    count = collections.Counter(sc.ints(n))
    odd = sum(1 for c in count.values() if c % 2)
    print(n - odd // 2)

def main():
    sc = Scanner()
    solve(sc)

if __name__ == "__main__":
    main()
