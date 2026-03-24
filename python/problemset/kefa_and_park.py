#!/usr/bin/env python3
import sys

data = iter(sys.stdin.read().split())
n, m = int(next(data)), int(next(data))
cat_nodes = []
adj_list = [[] for _ in range(n+1)]
for _ in range(n):
    cat_nodes.append(int(next(data)))
for _ in range(n-1):
    i, j = int(next(data)), int(next(data))
    adj_list[i].append(j)
    adj_list[j].append(i)
print("Cat Nodes: ", cat_nodes)
print("Adjacency: ", adj_list)
