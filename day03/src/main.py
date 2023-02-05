#!/usr/bin/env python

with open("./data/input.txt") as input:
    lines = input.readlines()

len_line = len(lines[0].strip())
len_data = len(lines)

counters = [0]*len_line
for line in lines:
    for i, c in enumerate(line.strip()):
        counters[i] += int(c)

gamma = int("".join([str(v // (len_data // 2)) for v in counters]), base=2)
epsilon = 2**len_line - 1 - gamma

print(f"Part 1: {gamma}, {epsilon}, {gamma * epsilon}")