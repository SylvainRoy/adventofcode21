#!/usr/bin/env python


class Node:
    def __init__(self, name):
        self.name = name
        self.vertices = []

    def add_vertix(self, node):
        if node.name != "start":
            self.vertices.append(node)


def find_paths(origin, target, path, usedtwice):
    if origin == target:
        return 1
    if origin.name[0] == origin.name[0].lower():
        passed = len([x for x in path if x == origin.name])
        if usedtwice:
            if passed > 0:
                return 0
        else:
            if passed > 1:
                return 0
            elif passed > 0:
                usedtwice = True
    path.append(origin.name)
    out = 0
    for destination in origin.vertices:
        out += find_paths(destination, target, path, usedtwice)
    path.pop()
    return out


def main():
    nodes = {}
    with open("./data/input.txt") as finput:
        for line in finput.readlines():
            el = line.split("-")
            origin = el[0].strip()
            destination = el[1].strip()
            orgnode = nodes.setdefault(origin, Node(origin))
            destnode = nodes.setdefault(destination, Node(destination))
            orgnode.add_vertix(destnode)
            destnode.add_vertix(orgnode)
    start = nodes["start"]
    end = nodes["end"]

    # part 1
    print("Part 1 - paths:", find_paths(start, end, [], True))

    # part 2
    print("Part 2 - paths:", find_paths(start, end, [], False))


if __name__ == "__main__":
    main()
