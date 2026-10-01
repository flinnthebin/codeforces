import sys
import collections
class scanner:
    __slots__: ("_it",)
    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def ints(self, n):
        return [int(next(self._it)) for _ in range(n)]


def solve(sc, out):
    n = sc.i()
    a = sorted(sc.ints(n))
    c = collections.Counter(a)
    cond = all(a[i] == a[i+1] for i in range(1, n-1, 2))
    out.append("YES" if cond else "NO")

def main():
    sc = scanner()
    out = []
    for _ in range(sc.i()): #t
        solve(sc, out)
    sys.stdout.write("\n".join(out) + "\n")

if __name__ == "__main__":
    main()
