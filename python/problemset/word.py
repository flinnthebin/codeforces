#!/usr/bin/env python3

print((lambda w: w.upper() if sum(c.isupper() for c in w) > sum(c.islower() for c in w) else w.lower())(input()))
