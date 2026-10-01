import sys
from heapq import heapify, heappushpop

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
    m = sc.i()
    a = sc.ints(n)

    if m == 1:
        out.append(max(a))
        return
 
    heap = [-x for x in a[:m - 1]]
    heapify(heap)
    s = sum(a[:m - 1])

    best = -(1 << 62)
    for j in range(m - 1, n):
        v = a[j]
        cand = m * v - s
        if cand > best:
            best = cand
        s += v + heappushpop(heap, -v)

    out.append(best)


 
def main():
    sc = Scanner()
    out = []
    for _ in range(sc.i()):
        solve(sc, out)
    sys.stdout.write("\n".join(str(x) for x in out) + "\n")

if __name__ == "__main__":
    main()
