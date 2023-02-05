#!/usr/bin/env pyton

from queue import PriorityQueue


class Cell:
    def __init__(self, i, j, risk, score, parent):
        self.i = i
        self.j = j
        self.risk = risk
        self.score = score
        self.parent = parent
        self.childs = []

    def __lt__(self, other):
        return (self.i, self.j) < (other.i, other.j)


def dijkstra(graph, start, end):
    start.score = 0
    queue = PriorityQueue()
    for node in graph:
        if node != start:
            node.score = float("inf")
            node.parent = None
        queue.put((node.score, node))
    while not queue.empty():
        (prio, cell) = queue.get()
        for child in cell.childs:
            score = cell.score + child.risk
            if score < child.score:
                child.score = score
                child.parent = cell
                child = queue.put((child.score, child))
    return end.score


def main():
    #
    # Part 1
    #

    # Build map
    map = []
    with open("./data/input.txt") as f:
        for i, line in enumerate(f.readlines()):
            row = []
            for j, risk in enumerate(line.strip()):
                cell = Cell(i, j, int(risk), None, None)
                row.append(cell)
            map.append(row)
    # Update nodes with their childs
    for i in range(len(map)):
        for j in range(len(map[0])):
            for (ii, jj) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
                if 0 <= ii < len(map) and 0 <= jj < len(map[0]):
                    map[i][j].childs.append(map[ii][jj])
    # Shortest path
    graph = [cell for row in map for cell in row]
    score = dijkstra(graph, map[0][0], map[-1][-1])
    print("Part 1 - shortest=", score)

    #
    # Part 2
    #

    # Build new map
    map5 = []
    for mi in range(5):
        for i in range(len(map)):
            row = []
            map5.append(row)
            for mj in range(5):
                for j in range(len(map[0])):
                    ii = mi * len(map) + i
                    jj = mj * len(map[0]) + j
                    cell = Cell(
                        ii, jj, (map[i][j].risk - 1 + mi + mj) % 9 + 1, None, None
                    )
                    row.append(cell)
    # Update nodes with their childs
    for i in range(len(map5)):
        for j in range(len(map5[0])):
            for (ii, jj) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
                if 0 <= ii < len(map5) and 0 <= jj < len(map5[0]):
                    map5[i][j].childs.append(map5[ii][jj])
    # Find shortest path
    graph5 = [cell for row in map5 for cell in row]
    score = dijkstra(graph5, map5[0][0], map5[-1][-1])
    print("Part 2 - shortest=", score)


if __name__ == "__main__":
    main()
