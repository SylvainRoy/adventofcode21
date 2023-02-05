#!/usr/bin/env python

import re
from math import copysign


class Probe:
    def __init__(self, vx, vy):
        self.vx = vx
        self.vy = vy
        self.x = 0
        self.y = 0

    def step(self):
        self.x += self.vx
        self.y += self.vy
        self.vx -= int(copysign(min(1, abs(self.vx)), self.vx))
        self.vy -= 1

    def missed(self, target):
        return (
            (self.vx > 0 and self.x > target.xmax)
            or (self.vx < 0 and self.x < target.xmin)
            or (self.vy <= 0 and self.y < target.ymin)
        )

    def within(self, target):
        return (target.xmin <= self.x <= target.xmax) and (
            target.ymin <= self.y <= target.ymax
        )

    def __str__(self):
        return f"Probe[p={self.x},{self.y} / v={self.vx},{self.vy}]"

    def fire(self, target, debug=False):
        max_y = self.y
        while not (self.missed(target) or self.within(target)):
            self.step()
            if debug:
                print(self)
            max_y = max(max_y, self.y)
        if self.within(target):
            return max_y
        else:
            return -1


class Target:
    def __init__(self, xmin, xmax, ymin, ymax):
        self.xmin = xmin
        self.xmax = xmax
        self.ymin = ymin
        self.ymax = ymax

    def __str__(self):
        return f"Target[{self.xmin} <= x <= {self.xmax} and {self.ymin} <= y <= {self.ymax}]"


def main():

    # read input
    with open("./data/input.txt") as f:
        input = f.read()
    m = re.search(r"x=([^.]+)\.\.([^,]+).*y=([^.]+)\.\.(.+)", input)
    xmin, xmax, ymin, ymax = (int(x) for x in m.groups())

    #
    # Part 1
    #
    target = Target(xmin, xmax, ymin, ymax)
    direction = int(copysign(1, (target.xmax - target.xmin) / 2))
    max_y = 0
    vmax = (0, 0)
    for vx in range(100):
        for vy in range(100):
            y = Probe(vx, vy).fire(target)
            if y > max_y:
                max_y = y
                vmax = (vx, vy)
    print(f"Part 1 - max(y) = {max_y} for v={vmax}")

    #
    # Part 2
    #
    v = []
    for vx in range(300):
        for vy in range(-300, 300):
            y = Probe(vx, vy).fire(target)
            if y > -1:
                v.append((vx, vy))
    print(f"Part 2 - num velocities = {len(v)}")


if __name__ == "__main__":
    main()
