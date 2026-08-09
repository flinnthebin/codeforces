import sys


class Scanner:
    __slots__ = ("_it",)

    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def s(self):
        return next(self._it).decode()


def solve(sc, out):
    n, k = sc.i(), sc.i()
    s = sc.s()
    nxt = s[1:] + s[0]
    a, b = int(s[0::2], 2), int(nxt[0::2], 2)
    red = bin(a & (a ^ b)).count("1")
    c, d = int(s[1::2], 2), int(nxt[1::2], 2)
    red += bin(c & d).count("1")
    print(bin(c & d))

    out.append(f"{red} {s.count('1') - red}")


def main():
    sc = Scanner()
    out = []
    for _ in range(sc.i()):
        solve(sc, out)
    sys.stdout.write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
