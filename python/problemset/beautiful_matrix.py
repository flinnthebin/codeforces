#!/usr/bin/env python3

import sys

lines = sys.stdin.read().splitlines()
matrix = [list(map(int, line.split())) for line in lines]
row_idx = [i for i in range(len(matrix)) if 1 in matrix[i]][0]
col_idx = [i for i in range(len(matrix[row_idx])) if matrix[row_idx][i] == 1][0]
print(abs(row_idx - 2) + abs(col_idx - 2))
