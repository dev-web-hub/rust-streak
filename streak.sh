#!/bin/zsh
# Rust streak daily commit helper

# Count commits to set streak day
DAY=$(($(git rev-list --count HEAD) + 1))

# Allow you to add a short description
if [ $# -eq 0 ]; then
  MSG="progress"
else
  MSG="$*"
fi

git add .
git commit -m "Day $DAY: Rust streak update — $MSG"
git push origin main
