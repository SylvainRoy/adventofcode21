#!/usr/bin/env python

with open("./data/input.txt") as input:
    lines = input.readlines()

horizontal = 0
depth = 0

for line in lines:
    dir, dist = line.split()
    dist = int(dist)
    if dir == "forward":
        horizontal += dist
    elif dir == "down":
        depth += dist
    elif dir == "up":
        depth -= dist

print(f"{depth} * {horizontal} = {horizontal * depth}")
