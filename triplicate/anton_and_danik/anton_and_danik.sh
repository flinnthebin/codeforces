#!/usr/bin/env bash

read n
read s

acnt=0
dcnt=0

for (( i=0;i<${#s};i++ )); do
  c="${s:$i:1}"
  if [[ "$c" == "A" ]]; then
    ((acnt++))
  fi
  if [[ "$c" == "D" ]]; then
    ((dcnt++))
  fi
done

if [[ "$acnt" == "$dcnt" ]]; then
  echo "Friendship"
elif [[ "$acnt" > "$dcnt" ]]; then
  echo "Anton"
else
  echo "Danik"
fi
