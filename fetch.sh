#!/usr/bin/env bash

YEAR="2023"

# Check if AOC_SESSION variable is set
if [ -z ${AOC_SESSION+x} ]; then
    echo "AOC_SESSION is unset";
    echo "run: export AOC_SESSION=<YOUR_AOC_SESSION_KEY>";
    echo "You can get you session key from your browser cookies for adventofcode.com";
    exit 1
fi

# Check if no arguments were provided and prompt for a day number
if [ $# -eq 0 ]; then
    echo "No arguments were provided"
    echo "Which day you want to fetch?"
    read DAY
else
    DAY="$1"
fi

# Check that day is a number and within range
if ! [[ "$DAY" =~ ^[0-9]+$ ]] || [ "$DAY" -lt 1 ] || [ "$DAY" -gt 31 ]; then
    echo "You must enter a number between 1 and 31"
    echo $DAY
    exit 1
fi


output="input/$(printf '%02d' $DAY).txt"

if [ -f "$output" ]; then
    echo "Input file for day $DAY already exists"
    exit 0
fi

curl -s "https://adventofcode.com/$YEAR/day/$DAY/input" \
    --cookie "session=$AOC_SESSION" \
    -o "$output"
