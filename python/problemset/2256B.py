import sys

class Scanner:
    __slots__ = ("_it",)

    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def strs(self, n):
        return [next(self._it).decode() for _ in range(n)]

def solve(vec, out):
    for s in vec:
        res = 1
        for start in (0,1):
            keys = {(ord(c) - 48 + j) & 1
                    for j, c in enumerate(s[start::2]) if c != "?"}
            res *= 2 - len(keys)
        out.append(res)

def main():
    sc = Scanner()
    t = sc.i()
    vec = sc.strs(2*t)[1::2]
    out = []
    solve(vec, out)
    sys.stdout.write("\n".join(map(str, out)) + "\n")

if __name__ == "__main__":
    main()
