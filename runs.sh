#!/bin/sh

cargo update
for t in `seq 0 9` ; do
    taskset --cpu-list 0-$t cargo run --release --example find -- $t
done
./curves.py 100000000 > 1.dat
# ./curves.py 200000000 > 2.dat
# ./curves.py 300000000 > 3.dat
gnuplot p.gp
