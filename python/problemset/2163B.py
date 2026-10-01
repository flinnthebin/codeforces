import sys
import string
class Scanner:
    __slots__ = ("_it",)
    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def s(self):
        return next(self._it).decode()

    def ints(self,n):
        return [int(next(self._it)) for _ in range(n)]

def solve(sc, out):
    n = sc.i()
    p = sc.ints(n)
    x = sc.s()
    pos1 = p.index(1)
    posn = p.index(n)
    for bad in (0, n - 1, pos1, posn):
        if x[bad] == '1':
            out.append("-1")
            return
    a, b = sorted((pos1, posn))
    ops = [(0, a), (0, b), (a, b), (a, n - 1), (b, n - 1)]
    out.append("5")
    for l, r in ops:
        out.append(f"{l + 1} {r + 1}")

def main():
    sc = Scanner()
    out = []
    for _ in range(sc.i()): #t
        solve(sc, out)
    sys.stdout.write("\n".join(out) + "\n")

if __name__ == "__main__":
    main()
