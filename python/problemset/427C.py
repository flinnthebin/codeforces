import sys

MOD = 1000000007

class Scanner:
    __slots__: ("_it",)
    def __init__(self, stream=None):
        self._it = iter((stream or sys.stdin.buffer).read().split())
    def i(self):
        return int(next(self._it))
    def ints(self, n):
        return [int(next(self._it)) for _ in range(n)]
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

def dfs_order(src, adj, visited):
    order = []
    stack = [(src, -1, True)]
    while stack:
        v, p, entering = stack.pop()
        if entering:
            if visited[v]:
                continue
            visited[v] = True
            order.append((v, p, True))
            stack.append((v, p, False))
            for u in adj[v]:
                if not visited[u]:
                    stack.append((u, v, True))
        else:
            order.append((v, p, False))
    return order

def solve(sc):
    n = sc.i()
    cost = sc.ints(n)
    m = sc.i()
    g = sc.graph(n,m,directed=True)
    rg = [[] for _ in range(n)]
    for u in range(n):
        for v in g[u]:
            rg[v].append(u)
    visited = [False] * n
    finish = []
    for s in range(n):
        if not visited[s]:
            for v, _, entering in dfs_order(s, g, visited):
                if not entering:
                    finish.append(v)
    assigned = [False] * n
    total = 0
    ways = 1
    for s in reversed(finish):
        if assigned[s]:
            continue
        comp = [cost[v] for v, _, entering in dfs_order(s, rg, assigned) if entering]
        best = min(comp)
        total += best
        ways = ways * comp.count(best) % MOD

    print(total, ways)


def main():
    sc = Scanner()
    solve(sc)

if __name__ == "__main__":
    main()
