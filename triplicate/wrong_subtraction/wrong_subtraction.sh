#!/usr/bin/env bash

read n k

for i in $(seq 0 "$((k - 1))")
do
  if [ $((n % 10)) -eq 0 ]; then
    n=$((n / 10))
  else
    n=$((n - 1))
  fi
done
echo "$n"
