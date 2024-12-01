#!/bin/bash

if [ -z "$1" ]; then
  echo "Provide a day number."
  echo "usage: $0 DAY"
  exit 1
fi

if [ -z "$AOC_TOKEN" ]; then
  echo "No session token."
  exit 1
fi

URL="https://adventofcode.com/2024/day/$1/input"
curl -A "https://github.com/schmidtwmark/advent-of-code2024/blob/main/fetch.sh by mark.schmidt@hey.com" $URL --cookie $AOC_TOKEN > inputs/$1.txt
touch samples/$1.txt
cp template.rs src/bin/$1.rs
sed -i '' -e "s/aaaaa/$1/g"  src/bin/$1.rs
