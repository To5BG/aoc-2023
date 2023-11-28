#!/bin/bash
set -euo pipefail

if [[ $# != 1 ]]; then
  echo No day provided.
  echo usage: $0 "<day>"
  exit 1
fi

if [[ ! "$1" =~ ^([1-9]|1[0-9]|2[0-5])$ ]]; then
  echo Not a valid day: "$1"
  exit 1
fi

if [[ -z "${AOC_SESSION-""}" ]]; then
  echo \$AOC_SESSION not set
  exit 1
fi

curl -k "https://adventofcode.com/2022/day/$1/input" --cookie "session=$AOC_SESSION" > ./inputs/$1.in