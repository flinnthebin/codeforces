"""
============================ CP TEMPLATE (Python 3) ============================
Copy-paste sheet for Codeforces. Delete what you don't use.

QUICK NOTES BEFORE YOU SUBMIT
  - Prefer PyPy 3-64 on CF for loop-heavy code (5-20x faster). Stick with
    CPython when the work is big-int arithmetic (pow, //, %) - PyPy is slower
    there.
  - Never use bare input() in a loop. Use the Scanner below, or
    input = sys.stdin.readline.
  - Never print() in a loop. Buffer into a list, join once at the end.
  - dict/set with int or str keys are hackable on CF (adaptive anti-hash tests).
    If a problem invites hacks, salt your keys - see HASH SAFETY below.
  - Recursion is slow and shallow. Write iterative DFS. If you must recurse,
    see the thread trick in RECURSION below.
================================================================================
"""

import sys
from bisect import bisect_left, bisect_right, insort
from collections import Counter, defaultdict, deque
from heapq import heapify, heappop, heappush
from itertools import accumulate, combinations, permutations
from math import comb, gcd, isqrt, lcm, perm

MOD = 998244353
# MOD = 10 ** 9 + 7
INF = float("inf")

# ============================== FAST IO =======================================


class Scanner:
    """Whitespace-token reader over the whole of stdin. Mirrors the Rust one.

    If a problem needs whole lines with spaces intact, skip this and use:
        lines = sys.stdin.read().splitlines()
    """

    __slots__ = ("_it",)

    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())

    def tok(self):
        return next(self._it)          # raw bytes

    def i(self):
        return int(next(self._it))     # int() eats bytes fine

    def f(self):
        return float(next(self._it))

    def s(self):
        return next(self._it).decode()

    def ints(self, n):
        return [int(next(self._it)) for _ in range(n)]

    def floats(self, n):
        return [float(next(self._it)) for _ in range(n)]

    def strs(self, n):
        return [next(self._it).decode() for _ in range(n)]

    def pairs(self, n):
        it = self._it
        return [(int(next(it)), int(next(it))) for _ in range(n)]

    def grid(self, r):
        """r rows of a character grid -> list of str."""
        return [next(self._it).decode() for _ in range(r)]

    def graph(self, n, m, directed=False, one_indexed=True):
        """Adjacency list from m edge lines."""
        adj = [[] for _ in range(n)]
        d = 1 if one_indexed else 0
        it = self._it
        for _ in range(m):
            u = int(next(it)) - d
            v = int(next(it)) - d
            adj[u].append(v)
            if not directed:
                adj[v].append(u)
        return adj

    def wgraph(self, n, m, directed=False, one_indexed=True):
        """Weighted adjacency list: adj[u] holds (weight, v) - heap-friendly."""
        adj = [[] for _ in range(n)]
        d = 1 if one_indexed else 0
        it = self._it
        for _ in range(m):
            u = int(next(it)) - d
            v = int(next(it)) - d
            w = int(next(it))
            adj[u].append((w, v))
            if not directed:
                adj[v].append((w, u))
        return adj


# ============================== RECURSION =====================================
# Iterative is better. But when the recursion is genuinely easier to write:
#
#   import threading
#   sys.setrecursionlimit(1 << 25)
#   threading.stack_size(1 << 27)
#   t = threading.Thread(target=main)
#   t.start(); t.join()
#
# On PyPy this trick doesn't help much - PyPy handles deep recursion poorly
# either way. Convert to an explicit stack for anything past ~10^5 depth.

# ============================= HASH SAFETY ====================================
# Python's hash(int) is the identity for small ints, so a hacker can collide
# your dict on purpose. When it matters:
#
#   import random
#   RND = random.getrandbits(32)
#   cnt[x ^ RND] += 1
#
# Or just use sorted lists + bisect, or index into a plain list after
# coordinate compression.

# ================================ MATH ========================================
# gcd, lcm, isqrt, comb, perm are imported from math above.
# Modular inverse: pow(a, -1, m)   (Python 3.8+, works for any coprime m)
# Modular power:   pow(a, e, m)
# Integer sqrt:    isqrt(n)        (exact, no float error)
# Ceiling divide:  -(-a // b)

# n-th Term Formula: \(A_n = a_1 + (n - 1) \times d\)
def nth_term(a1, d, n):
    return a1 + (n - 1) * d

# Sum of n Terms Formula: \(S_n = \frac{n}{2} \times (2a_1 + (n - 1) \times d)\)
def arithmetic_sum(a1, d, n):
    return (n / 2) * (2 * a1 + (n - 1) * d)


def ext_gcd(a, b):
    """Returns (g, x, y) with a*x + b*y == g == gcd(a, b).

    Note the order differs from the Rust version, which returns (x, y, g).
    """
    old_r, r = a, b
    old_x, x = 1, 0
    old_y, y = 0, 1
    while r:
        q = old_r // r
        old_r, r = r, old_r - q * r
        old_x, x = x, old_x - q * x
        old_y, y = y, old_y - q * y
    return old_r, old_x, old_y


def crt(r1, m1, r2, m2):
    """Smallest x >= 0 with x = r1 (mod m1), x = r2 (mod m2). None if impossible.

    Returns (x, lcm(m1, m2)).
    """
    g, p, _ = ext_gcd(m1, m2)
    if (r2 - r1) % g:
        return None
    l = m1 // g * m2
    x = (r1 + (r2 - r1) // g % (m2 // g) * p % (m2 // g) * m1) % l
    return x, l


def divisors(n):
    """All divisors of n, unsorted. O(sqrt n)."""
    small, large = [], []
    d = 1
    while d * d <= n:
        if n % d == 0:
            small.append(d)
            if d != n // d:
                large.append(n // d)
        d += 1
    return small + large[::-1]


def factorize(n):
    """Prime factorisation of a single n as {prime: exponent}. O(sqrt n)."""
    f = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            f[d] = f.get(d, 0) + 1
            n //= d
        d += 1 if d == 2 else 2
    if n > 1:
        f[n] = f.get(n, 0) + 1
    return f


def mat_mul(a, b, mod=MOD):
    n, k, m = len(a), len(b), len(b[0])
    c = [[0] * m for _ in range(n)]
    for i in range(n):
        ai, ci = a[i], c[i]
        for t in range(k):
            if ai[t]:
                v, bt = ai[t], b[t]
                for j in range(m):
                    ci[j] = (ci[j] + v * bt[j]) % mod
    return c


def mat_pow(a, e, mod=MOD):
    """For linear recurrences. a must be square."""
    n = len(a)
    r = [[int(i == j) for j in range(n)] for i in range(n)]
    while e:
        if e & 1:
            r = mat_mul(r, a, mod)
        a = mat_mul(a, a, mod)
        e >>= 1
    return r


# ============================= PREFIX SUMS ====================================
# 1D: use itertools.accumulate.
#   ps = [0] + list(accumulate(a))      -> sum(a[l:r]) == ps[r] - ps[l]


def prefix_2d(g):
    """g is rows x cols of ints. Returns (rows+1) x (cols+1) prefix sums."""
    r, c = len(g), len(g[0])
    ps = [[0] * (c + 1) for _ in range(r + 1)]
    for i in range(r):
        row, prev, cur = g[i], ps[i], ps[i + 1]
        s = 0
        for j in range(c):
            s += row[j]
            cur[j + 1] = prev[j + 1] + s
    return ps


def rect_sum(ps, r1, c1, r2, c2):
    """Inclusive corners, 0-indexed into the original grid."""
    return ps[r2 + 1][c2 + 1] - ps[r1][c2 + 1] - ps[r2 + 1][c1] + ps[r1][c1]


# ============================ BINARY SEARCH ===================================
# Sorted arrays: bisect_left(a, x) is lower_bound, bisect_right(a, x) is
# upper_bound. Both return len(a) when nothing qualifies - no Option wrapper.


def bsearch(lo, hi, pred):
    """First x in [lo, hi] with pred(x) True, assuming F...F T...T.

    Returns hi when pred never holds, so pass hi as a sentinel one past the
    real range - same idea as bisect returning len(a).
    """
    while lo < hi:
        mid = (lo + hi) // 2
        if pred(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo


def bsearch_last(lo, hi, pred):
    """Last x in [lo, hi] with pred(x) True, assuming T...T F...F.

    Returns lo - 1 if pred is never True.
    """
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if pred(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo if pred(lo) else lo - 1


def bsearch_float(lo, hi, pred, iters=100):
    """Fixed iteration count beats an epsilon condition - can't hang."""
    for _ in range(iters):
        mid = (lo + hi) / 2
        if pred(mid):
            hi = mid
        else:
            lo = mid
    return lo


# ================================ SIEVE =======================================


def sieve(n):
    """All primes <= n."""
    if n < 2:
        return []
    is_p = bytearray([1]) * (n + 1)
    is_p[0:2] = b"\x00\x00"
    for i in range(2, isqrt(n) + 1):
        if is_p[i]:
            is_p[i * i :: i] = bytearray((n - i * i) // i + 1)
    return [i for i in range(2, n + 1) if is_p[i]]


def spf_sieve(n):
    """Smallest prime factor table. spf[x] lets you factorise x in O(log x)."""
    spf = list(range(n + 1))
    for i in range(2, isqrt(n) + 1):
        if spf[i] == i:
            for j in range(i * i, n + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def factorize_spf(x, spf):
    f = {}
    while x > 1:
        p = spf[x]
        while x % p == 0:
            f[p] = f.get(p, 0) + 1
            x //= p
    return f


# ================================ GRAPHS ======================================


def bfs(src, adj):
    """Unweighted distances. -1 for unreachable, matching the Rust version."""
    dist = [-1] * len(adj)
    dist[src] = 0
    q = deque([src])
    while q:
        v = q.popleft()
        d = dist[v] + 1
        for u in adj[v]:
            if dist[u] < 0:
                dist[u] = d
                q.append(u)
    return dist


def dfs(src, adj):
    """Iterative. Returns the visited mask."""
    seen = [False] * len(adj)
    stack = [src]
    seen[src] = True
    while stack:
        v = stack.pop()
        for u in adj[v]:
            if not seen[u]:
                seen[u] = True
                stack.append(u)
    return seen


def dfs_order(src, adj):
    """Iterative DFS emitting (node, entering) events - use this when you need
    real pre/post-order work, e.g. subtree aggregation on a rooted tree."""
    order = []
    stack = [(src, -1, True)]
    while stack:
        v, p, entering = stack.pop()
        if entering:
            order.append((v, p, True))
            stack.append((v, p, False))
            for u in adj[v]:
                if u != p:
                    stack.append((u, v, True))
        else:
            order.append((v, p, False))
    return order


def dijkstra(src, adj):
    """adj[u] holds (weight, v) pairs. Returns dist with INF for unreachable."""
    dist = [INF] * len(adj)
    dist[src] = 0
    pq = [(0, src)]
    while pq:
        d, v = heappop(pq)
        if d > dist[v]:
            continue
        for w, u in adj[v]:
            nd = d + w
            if nd < dist[u]:
                dist[u] = nd
                heappush(pq, (nd, u))
    return dist


def toposort(adj):
    """Kahn. Returns None if the digraph has a cycle."""
    n = len(adj)
    indeg = [0] * n
    for vs in adj:
        for v in vs:
            indeg[v] += 1
    q = deque(v for v in range(n) if indeg[v] == 0)
    order = []
    while q:
        v = q.popleft()
        order.append(v)
        for u in adj[v]:
            indeg[u] -= 1
            if indeg[u] == 0:
                q.append(u)
    return order if len(order) == n else None


# ================================= DSU ========================================


class DSU:
    __slots__ = ("p", "sz", "comps")

    def __init__(self, n):
        self.p = list(range(n))
        self.sz = [1] * n
        self.comps = n

    def find(self, x):
        p = self.p
        root = x
        while p[root] != root:
            root = p[root]
        while p[x] != root:              # path compression, no recursion
            p[x], x = root, p[x]
        return root

    def union(self, a, b):
        a, b = self.find(a), self.find(b)
        if a == b:
            return False
        if self.sz[a] < self.sz[b]:
            a, b = b, a
        self.p[b] = a
        self.sz[a] += self.sz[b]
        self.comps -= 1
        return True

    def same(self, a, b):
        return self.find(a) == self.find(b)

    def size(self, a):
        return self.sz[self.find(a)]


# =============================== FENWICK ======================================


class Fenwick:
    """1-indexed point-update / prefix-sum BIT."""

    __slots__ = ("n", "t")

    def __init__(self, n):
        self.n = n
        self.t = [0] * (n + 1)

    @classmethod
    def build(cls, a):
        """O(n) construction from a 0-indexed list."""
        f = cls(len(a))
        t = f.t
        t[1:] = a
        for i in range(1, f.n + 1):
            j = i + (i & -i)
            if j <= f.n:
                t[j] += t[i]
        return f

    def add(self, i, v):
        while i <= self.n:
            self.t[i] += v
            i += i & -i

    def sum(self, i):
        """Prefix sum of 1..=i."""
        s = 0
        while i > 0:
            s += self.t[i]
            i -= i & -i
        return s

    def range_sum(self, l, r):
        """Inclusive 1-indexed l..=r."""
        return self.sum(r) - self.sum(l - 1)

    def kth(self, k):
        """Smallest i with prefix_sum(i) >= k. Needs non-negative values.
        Turns the BIT into an order-statistic set in O(log n)."""
        pos, pw = 0, 1 << self.n.bit_length()
        while pw:
            nxt = pos + pw
            if nxt <= self.n and self.t[nxt] < k:
                pos = nxt
                k -= self.t[pos]
            pw >>= 1
        return pos + 1


# ============================== SEGMENT TREE ==================================


class SegTree:
    """Iterative bottom-up. Works for any associative f (non-commutative too)."""

    __slots__ = ("n", "t", "f", "e")

    def __init__(self, a, f=min, e=INF):
        self.n = n = len(a)
        self.f, self.e = f, e
        self.t = t = [e] * (2 * n)
        t[n:] = a
        for i in range(n - 1, 0, -1):
            t[i] = f(t[2 * i], t[2 * i + 1])

    def set(self, i, v):
        t, f = self.t, self.f
        i += self.n
        t[i] = v
        i >>= 1
        while i:
            t[i] = f(t[2 * i], t[2 * i + 1])
            i >>= 1

    def get(self, i):
        return self.t[i + self.n]

    def query(self, l, r):
        """Half-open [l, r)."""
        t, f = self.t, self.f
        resl = resr = self.e
        l += self.n
        r += self.n
        while l < r:
            if l & 1:
                resl = f(resl, t[l])
                l += 1
            if r & 1:
                r -= 1
                resr = f(t[r], resr)
            l >>= 1
            r >>= 1
        return f(resl, resr)


class SparseTable:
    """Static idempotent range queries (min/max/gcd) in O(1). Build O(n log n)."""

    __slots__ = ("f", "t")

    def __init__(self, a, f=min):
        self.f = f
        self.t = [list(a)]
        n, j = len(a), 1
        while (1 << j) <= n:
            prev, step = self.t[-1], 1 << (j - 1)
            self.t.append([f(prev[i], prev[i + step]) for i in range(n - (1 << j) + 1)])
            j += 1

    def query(self, l, r):
        """Half-open [l, r), r > l."""
        j = (r - l).bit_length() - 1
        return self.f(self.t[j][l], self.t[j][r - (1 << j)])


# ============================ COMBINATORICS ===================================
# No ModInt here - Python ints don't overflow, so just write `% MOD` and move on.
# For one-off values with no modulus, math.comb / math.perm are exact and fast.


class Comb:
    """Factorial tables mod a prime. Build once, query O(1)."""

    __slots__ = ("mod", "fact", "ifact")

    def __init__(self, n, mod=MOD):
        self.mod = mod
        fact = [1] * (n + 1)
        for i in range(1, n + 1):
            fact[i] = fact[i - 1] * i % mod
        ifact = [1] * (n + 1)
        ifact[n] = pow(fact[n], mod - 2, mod)
        for i in range(n, 0, -1):
            ifact[i - 1] = ifact[i] * i % mod
        self.fact, self.ifact = fact, ifact

    def ncr(self, n, r):
        if r < 0 or r > n:
            return 0
        return self.fact[n] * self.ifact[r] % self.mod * self.ifact[n - r] % self.mod

    def npr(self, n, r):
        if r < 0 or r > n:
            return 0
        return self.fact[n] * self.ifact[n - r] % self.mod

    def inv(self, n):
        """1/n mod p for 1 <= n <= table size."""
        return self.fact[n - 1] * self.ifact[n] % self.mod


# ================================ MISC ========================================


def compress(a):
    """Coordinate compression. Returns (mapped values, sorted distinct values)."""
    vals = sorted(set(a))
    idx = {v: i for i, v in enumerate(vals)}
    return [idx[v] for v in a], vals


def rle(s):
    """Run-length encode into [(value, count), ...]."""
    out = []
    for ch in s:
        if out and out[-1][0] == ch:
            out[-1][1] += 1
        else:
            out.append([ch, 1])
    return [(c, k) for c, k in out]


def transpose(grid):
    return ["".join(col) for col in zip(*grid)]


DIRS4 = ((-1, 0), (1, 0), (0, -1), (0, 1))
DIRS8 = ((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1))


def neighbours(r, c, rows, cols, dirs=DIRS4):
    for dr, dc in dirs:
        nr, nc = r + dr, c + dc
        if 0 <= nr < rows and 0 <= nc < cols:
            yield nr, nc


# ================================= MAIN =======================================


def solve(sc, out):
    n = sc.i()
    vectors = [sc.ints(2) for _ in range(n)]
    out.append(str(len(vectors)))


def main():
    sc = Scanner()
    out = []
    for _ in range(sc.i()):        # delete this loop for single-test problems
        solve(sc, out)
    sys.stdout.write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
