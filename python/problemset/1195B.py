import sys

class Scanner:
    __slots__ = ("_it",)
    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def i(self):
        return int(next(self._it))

    def ints(self,n):
        return [int(next(self._it)) for _ in range(n)]

def arithmetic_sum(a1, d, n):
    return (n/2) * (2*a1+(n-1)*d)

def solve(sc):
    n = sc.i()
    k = sc.i()
    for i in range(n+1):
        sum = arithmetic_sum(1,1,i)
        if sum - (n - i) == k:
            print(n - i)
            break

def main():
    sc = Scanner()
    solve(sc)


if __name__ == "__main__":
    main()
