#!/usr/bin/env python3
from sys import argv

def lines(filename):
    with open(filename) as data:
        for line in data:
            if line[0] == '#':
                continue
            content = line.split(",")
            nums = list(int(e) for e in content)
            yield nums

def main():
    log_files = argv[1:]
    algorithms_number = len(next(lines(log_files[1])))
    times = [[[] for _ in range(algorithms_number)] for _ in range(len(log_files))]
    for threads, data_file in enumerate(log_files):
        for timings in lines(data_file):
            for store, time in zip(times[threads], timings):
                store.append(time)
    # we create a 2d array
    # for each thread number the average running time for each algorithm
    averages = [
        [
            sum(algo) / len(algo)
            for algo in fixed_thread[1:]
        ]
        for fixed_thread in times
    ]
    medians = [
        [
            sorted(algo)[len(algo)//2]
            for algo in fixed_thread[1:]
        ]
        for fixed_thread in times
    ]
    average_speedups = [
        [
            sum(s/a for s, a in zip(fixed_thread[0], algo)) / len(algo)
            for algo in fixed_thread[1:]
        ]
        for fixed_thread in times
    ]
    median_speedups = [
        [
            sorted(s/a for s, a in zip(fixed_thread[0], algo))[len(algo)//2]
            for algo in fixed_thread[1:]
        ]
        for fixed_thread in times
    ]
    save("averages.csv", averages)
    save("medians.csv", medians)
    save("average_speedups.csv", average_speedups)
    save("median_speedups.csv", median_speedups)

def save(filename, data):
    with open(filename, "w") as f:
        for thread, times in enumerate(data):
            print("{},{}".format(thread+1, ",".join(str(t) for t in times)), file=f)


main()

