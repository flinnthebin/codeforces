import sys

lines = sys.stdin.read().splitlines()
n = int(lines[0])
vectors = [list(map(int, line.split())) for line in lines[1:]]

for v in vectors:
    v.sort()
    x = v[2] - v[0]
    y = v[1] + v[0] - v[0]
    if x <= y:
        print(x)
    else:
        print(y)
