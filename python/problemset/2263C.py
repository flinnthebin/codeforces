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
    n = sc.i()
    a = sc.ints(n)
 
    diff = [0] * (n + 1)
    for k in range(1, n + 1):
        lo = k * a[k - 1]
        if lo >= n:
            continue
        hi = min(n - 1, lo + k - 1)
        diff[lo] += 1
        diff[hi + 1] -= 1
 
    b = []
    cur = 0
    for y in range(n):
        cur += diff[y]
        if cur == 0:
            b.append(y)
 
    out.append(str(len(b)))
    out.append(" ".join(map(str, b)))

def main():
    sc = Scanner()
    out = []
    for _ in range(sc.i()):
        solve(sc, out)
    sys.stdout.write("\n".join(out) + "\n")

if __name__ == "__main__":
    main()
