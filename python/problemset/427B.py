import sys

class Scanner:
    __slots__ = ("_it",)
    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def tok(self):
        return next(self._it)

    def ints(self, n):
        return [int(next(self._it)) for _ in range(n)]

def solve(sc):
    n, t, c = sc.i(), sc.i(), sc.i()
    p = sc.ints(n)
    run = count = 0
    for x in p:
        run = run + 1 if x <= t else 0
        if run >= c:
            count += 1
    print(count)


def main():
    sc = Scanner()
    solve(sc)

if __name__ == "__main__":
    main()

