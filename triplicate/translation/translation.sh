#!/usr/bin/env bash

read first
read second

for ((i=0;i<${#second};i++)); do rev="${second:i:1}$rev"; done

if [[ "$first" == "$rev" ]]; then
  echo "YES"
else
  echo "NO"
fi
