#!/bin/bash

TODAY=$(date +%d)
YEARTODAY=$(date +%Y)

YEAR=$1
DAY=$2

if [[ -z "${YEAR}" ]] || [[ -z "${DAY}" ]]; then
    echo "ERROR: to few arguments, use scriptname [YEAR xxxx] [DAY : xx]" >&2
elif [[ ${#YEAR} != 4 ]] || [[ ${#DAY} != 2 ]]; then
    echo "[YEAR] or [DAY] wrong length, use scriptname [YEAR xxxx] [DAY : xx]" >&2
elif [[ $YEAR > $YEARTODAY ]] || [[ $DAY > $TODAY ]]; then
    echo "this AoC not released yet"
else  # No ERRORS
    

    #TODO Implement if failed to download the template
    AOC_FOLDER=$PWD

    mkdir -p "$PWD/$1/$2"
    cd "$PWD/$1/$2"

    # init cargo project
    cargo init --name "aoc_$YEAR_$DAY"

    cp "$AOC_FOLDER/template.rs" "$PWD/src/main.rs" -f

    #download the testing data
    cargo aoc input -y $YEAR -d $DAY

    # File actually named with only one digit 09 becomes 9.txt
    # While argument to cargo aoc uses 09
    if [[ "${DAY:0:1}" == 0 ]]; then 
        DAY="${DAY:1:1}"
        echo "day is $DAY"
    fi
    # move the file 
    mv "$PWD/input/$YEAR/day$DAY.txt" "$PWD/src/input.txt"
    rm "$PWD/input" -r

    #nvim "$PWD/src/input.txt"
fi
