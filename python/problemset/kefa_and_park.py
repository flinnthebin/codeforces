#!/usr/bin/env python3
import sys

data = iter(sys.stdin.read().split())
n, m = int(next(data)), int(next(data))
cat_nodes = [0]
adj_list = [[] for _ in range(n+1)]
for _ in range(n):
    cat_nodes.append(int(next(data)))
for _ in range(n-1):
    i, j = int(next(data)), int(next(data))
    adj_list[i].append(j)
    adj_list[j].append(i)

def dfs(n, m, adj_list, cat_nodes):
    valid_leaves = 0
    visited = [False] * (n + 1)
    stack = [(1, -1, 0)]
    while stack:
        u, p, streak = stack.pop()
        new_streak = streak + 1 if cat_nodes[u] == 1 else 0
        if new_streak > m: continue
        visited[u] = True
        is_leaf = True
        for v in adj_list[u]:
            if v != p:
                is_leaf = False
                stack.append((v, u, new_streak))
        if is_leaf and u != 1:
            valid_leaves += 1
    return(valid_leaves)

print(dfs(n, m, adj_list, cat_nodes))


