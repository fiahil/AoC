#!/bin/bash

for i in {1..25}
do
    mkdir -p d$i
    curl -s "https://adventofcode.com/2018/day/$i/input" \
         -H 'cookie: session=53616c7465645f5fefb70adacd3742f25318b7323f7e569c805864464e48a1f9804bd045388846240ed13b406229f497' > d$i/input
    echo "D$i [OK]"
done