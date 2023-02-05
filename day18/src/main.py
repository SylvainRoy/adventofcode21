#!/usr/bin/env python

import math
import functools
from copy import deepcopy


class Explosion:
    def __init__(self, left, center, right):
        self.left = left
        self.center = center
        self.right = right


class Split:
    def __init__(self, value):
        self.value = value


class Value:
    def __init__(self, value=None):
        self.value = value

    def parse(self, data, index):
        i = 0
        while data[index + i].isdigit():
            i += 1
        self.value = int(data[index : index + i], 10)
        return self, index + i

    def __repr__(self):
        return str(self.value)

    def explode(self, depth=0):
        return None

    def add_left(self, value):
        self.value += value.value

    def add_right(self, value):
        self.value += value.value

    def split(self):
        if self.value > 9:
            return Split(
                Pair(Value(int(self.value / 2)), Value(math.ceil(self.value / 2)))
            )
        else:
            return None

    def magnitude(self):
        return self.value


class Pair:
    def __init__(self, left=None, right=None):
        self.left = left
        self.right = right

    def parse(self, data, index):
        assert data[index] == "["
        index += 1
        self.left, index = parse(data, index)
        assert data[index] == ","
        index += 1
        self.right, index = parse(data, index)
        assert data[index] == "]"
        index += 1
        return self, index

    def __repr__(self):
        return f"[{self.left},{self.right}]"

    def explode(self, depth=0):
        if depth == 4:
            return Explosion(self.left, Value(0), self.right)
        exp = self.left.explode(depth + 1)
        if exp is not None:
            if exp.right is not None:
                self.right.add_left(exp.right)
                exp.right = None
            if exp.center is not None:
                self.left = exp.center
                exp.center = None
            return exp
        exp = self.right.explode(depth + 1)
        if exp is not None:
            if exp.left is not None:
                self.left.add_right(exp.left)
                exp.left = None
            if exp.center is not None:
                self.right = exp.center
                exp.center = None
            return exp
        return None

    def add_left(self, value):
        self.left.add_left(value)

    def add_right(self, value):
        self.right.add_right(value)

    def split(self):
        lsplit = self.left.split()
        if lsplit is not None:
            if lsplit.value is not None:
                assert isinstance(lsplit.value, Pair)
                self.left = lsplit.value
                lsplit.value = None
            return lsplit
        rsplit = self.right.split()
        if rsplit is not None:
            if rsplit.value is not None:
                assert isinstance(rsplit.value, Pair)
                self.right = rsplit.value
                rsplit.value = None
            return rsplit
        return None

    def magnitude(self):
        return 3 * self.left.magnitude() + 2 * self.right.magnitude()


def parse(data, index=0):
    if data[index] == "[":
        return Pair().parse(data, index)
    else:
        return Value().parse(data, index)


def reduce(sfnumber):
    while True:
        exp = sfnumber.explode()
        if exp is not None:
            continue
        spt = sfnumber.split()
        if spt is not None:
            continue
        break


def add_reduce(m, n):
    p = Pair(m, n)
    reduce(p)
    return p


def main():

    # Read input
    sfnumbers = []
    with open("./data/input.txt") as f:
        for line in f.readlines():
            sfnumbers.append(parse(line)[0])

    #
    # Part 1
    #
    p = functools.reduce(add_reduce, deepcopy(sfnumbers))
    print(f"Part 1 - Magnitude: {p.magnitude()}")

    #
    # Part 2
    #
    max_magnitude = 0
    for m in sfnumbers:
        for n in sfnumbers:
            if m == n:
                continue
            o = Pair(deepcopy(m), deepcopy(n))
            reduce(o)
            if max_magnitude < o.magnitude():
                max_magnitude = o.magnitude()
    print(f"Part 2 - Max magnitude: {max_magnitude}")


if __name__ == "__main__":
    main()
