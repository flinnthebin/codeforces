import sys

input = sys.stdin.read().split()
iterator = iter(input)

def next_token():
    return next(iterator)

try:
    cases = int(next_token())
    for _ in range(cases):
        x = int(next_token())
        for i in range(x):
            print(i + 1)
except StopIteration:
    pass
