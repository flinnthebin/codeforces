#!/usr/bin/env bash

read n

xcnt=0
ycnt=0
zcnt=0

for ((i=0; i<n; i++)); do
  read x y z
  (( xcnt += x ))
  (( ycnt += y ))
  (( zcnt += z ))
done

if [[ $xcnt -eq 0 && $ycnt -eq 0 && $zcnt -eq 0 ]]; then
  echo "YES"
else
  echo "NO"
fi
