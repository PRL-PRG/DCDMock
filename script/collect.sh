#!/bin/bash

number=0
wanted=1000000 #856411

for list in `find . -name '*.csv'`
do
    echo "$list"
    number_here=$(tail -n +2 "$list" | wc -l)

    number=$((number + number_here))
    echo "$number"
    echo

    tail -n +2 "$list" >> mock_commit_messages.csv

    if [ $number -ge $wanted ]
    then
        break
    fi
done
