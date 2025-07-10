#!/usr/bin/env python3

s = list(input())
res = []

for i in range(len(s)):
    if s[i] == 'h' and res.count('h') < 1:
        res.append(s[i])
    if s[i] == 'e' and 'h' in res and res.count('e') < 1:
        res.append(s[i])
    if s[i] == 'l' and 'e' in res and res.count('l') < 2:
        res.append(s[i])
    if s[i] == 'o' and 'l' in res and res.count('l') == 2 and res.count('o') < 1:
        res.append(s[i])

ans = ''.join(res)
print("YES" if ans == 'hello' else "NO")
