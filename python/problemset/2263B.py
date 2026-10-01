import sys

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
    n, k = sc.i(), sc.i()
    if k < n or k > 2 * n - 1:
        out.append("-1")
        return

    bsize = k - n + 1
    hi = k + 1
    rows = []

    row = list(range(1, bsize + 1))
    row += range(hi, hi + n - bsize); hi += n - bsize
    rows.append(row)

    for _ in range(1, bsize):
        row = [bsize + len(rows)]
        row += range(hi, hi + n - 1); hi += n - 1
        rows.append(row)

    v = 2 * bsize - 1
    for i in range(bsize, n):
        v += 1
        row = list(range(hi, hi + i)); hi += i
        row.append(v)
        row += range(hi, hi + n - i - 1); hi += n - i - 1
        rows.append(row)

    out.append("\n".join(" ".join(map(str, r)) for r in rows))

def main():
    sc = Scanner()
    out = []
    for _ in range(sc.i()):
        solve(sc, out)
    sys.stdout.write("\n".join(out) + "\n")

if __name__ == "__main__":
    main()
