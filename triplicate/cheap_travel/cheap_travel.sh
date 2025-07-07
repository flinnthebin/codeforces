#!/usr/bin/env bash

read n m a b

cost_a=$((n * a))
full_tix=$((n / m))
leftover=$((n % m))
cost_b=$((full_tix * b + leftover * a))
cost_c=$(( (((n + m - 1) / m) * b)))

echo $(( cost1 < cost2 ? (cost1 < cost3 ? cost1 : cost3) : (cost2 < cost3 ? cost2 : cost3) ))
