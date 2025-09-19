#!/bin/zsh
# Rust streak daily commit helper
# Calculates streak day from fixed start date: 2025-09-01

START_DATE="2025-09-01"
TODAY=$(date +%Y-%m-%d)
DAY=$(( ( $(date -j -f "%Y-%m-%d" "$TODAY" +%s) - $(date -j -f "%Y-%m-%d" "$START_DATE" +%s) ) / 86400 + 1 ))

if [ $# -eq 0 ]; then
  MSG="progress"
else
  MSG="$*"
fi

git add .
git commit -m "Day $DAY: Rust streak update — $MSG"
git push origin main
