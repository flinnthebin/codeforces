import sys

MOD = 998244353

class Scanner:
    __slots__: ("_it",)

    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def ints(self, n):
        return [int(next(self._it)) for _ in range(n)]

def solve(sc):
    n = sc.i()
    for _ in range(n):


def main():
    sc = Scanner()
    solve(sc)

if __name__ == "__main__":
    main()
