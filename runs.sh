#!/bin/bash
source $HOME/.profile

echo "all"
for t in `seq 0 15`
do
    taskset --cpu-list 0-$t cargo run --release --example all -- $(($t+1))
done
echo "find"
for t in `seq 0 15`
do
    taskset --cpu-list 0-$t cargo run --release --example find -- $(($t+1))
done
