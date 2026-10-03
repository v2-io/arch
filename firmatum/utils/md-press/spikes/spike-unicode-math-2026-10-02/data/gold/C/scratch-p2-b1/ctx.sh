#!/bin/bash
# usage: ctx.sh FILE NEEDLE [lines]
echo "=== $1 :: $2"
grep -n -B${3:-2} -A${3:-2} -F -- "$2" "$1" | cut -c1-400 | head -40
