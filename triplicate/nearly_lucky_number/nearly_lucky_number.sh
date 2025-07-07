#!/usr/bin/env bash

cnt=0
read n

for (( i=0; i<${#n};i++ )); do
  c="${n:$i:1}"
  if [[ "$c" == "4" || "$c" == "7" ]]; then
    ((cnt++))
  fi
done

for (( i=0; i<${#cnt};i++ )); do
  c="${cnt:$i:1}"
  if [[ "$c" != "4" && "$c" != "7" ]]; then
    echo "NO"
    exit 0
  fi
done

echo "YES"
