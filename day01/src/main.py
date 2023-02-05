#!/usr/bin/env python

with open("./data/input.txt") as input:
    lines = input.readlines()

depths = list(map(lambda x: int(x), lines))
couples = zip(depths[:-1], depths[1:])
count = len([x for (x, y) in couples if x < y])
print("count:", count)

depths3 = list(map(sum, zip(depths[:-2], depths[1:-1], depths[2:])))
couples = zip(depths3[:-1], depths3[1:])
count = len([x for (x, y) in couples if x < y])
print("count:", count)
