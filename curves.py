#!/usr/bin/env python3
from sys import argv

def lines(filename):
    with open(filename) as data:
        for line in data:
            if line[0] == '#':
                continue
            content = line.split()
            nums = list(int(e) for e in content)
            yield nums

def main():
    wanted = int(argv[1])
    algorithms_number = len(next(lines("0"))) - 2
    times = [[[] for _ in range(algorithms_number)] for _ in range(10)]
    for threads in range(10):
        for size, _, *timings in lines("{}".format(threads)):
            if size == wanted:
                for store, time in zip(times[threads], timings):
                    store.append(time)

    size = len(times[0][0])
    with open("averages_{}.dat".format(wanted), "w") as av:
        with open("medians_{}.dat".format(wanted), "w") as med:
            for i, timings in enumerate(times):
                for timing in timings:
                    timing.sort()
                averages = (sum(t) / size for t in timings)
                medians = (t[size//2] for t in timings)
                print(i+1, *averages, file=av)
                print(i+1, *medians, file=med)





main()

