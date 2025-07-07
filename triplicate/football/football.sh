#!/usr/bin/env bash

read s

zeroes=0
ones=0
res=false

for (( i=0;i<${#s};i++ )); do
  c="${s:$i:1}"
  if [[ "$c" == "0" ]]; then
    ((zeroes++))
    ones=0
  else
    ((ones++))
    zeroes=0
  fi
  if [[ $zeroes -ge 7 || $ones -ge 7 ]]; then
    res=true
  fi
done

if [[ $res == true ]]; then
  echo "YES"
else
  echo "NO"
fi
