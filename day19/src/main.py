import numpy as np
import functools as ft

from itertools import combinations, permutations

from numpy.typing import _256Bit

def mdis(m, sep=""):
    if len(m) == 9:
        return sep.join([str(l).replace(' ', '') for l in [m[:3], m[3:6], m[6:]]])
    elif len(m) == 3:
        return str(m)
    assert False

def vadd(u, v):
    assert len(u) == 3
    assert len(v) == 3
    return [u[0]+v[0], u[1]+v[1], u[2]+v[2]]

def vsub(u, v):
    assert len(u) == 3
    assert len(v) == 3
    return [u[0]-v[0], u[1]-v[1], u[2]-v[2]]

def mmat(d):
    assert len(d) == 3
    assert len(d[0]) == 3
    assert len(d[1]) == 3
    assert len(d[2]) == 3
    return [d[0][0], d[1][0], d[2][0], d[0][1], d[1][1], d[2][1], d[0][2], d[1][2], d[2][2]]

def mdet(m):
    return m[0] * (m[4]*m[8] - m[7]*m[5]) + m[3] * (m[1]*m[8] - m[7]*m[2]) + m[6] * (m[1]*m[5] - m[4]*m[2])

def mmult(m, n):
    assert len(m) == 9
    assert len(n) == 9
    return [
        m[0]*n[0] + m[1]*n[3] + m[2]*n[6],
        m[0]*n[1] + m[1]*n[4] + m[2]*n[7],
        m[0]*n[2] + m[1]*n[5] + m[2]*n[8],
        m[3]*n[0] + m[4]*n[3] + m[5]*n[6],
        m[3]*n[1] + m[4]*n[4] + m[5]*n[7],
        m[3]*n[2] + m[4]*n[5] + m[5]*n[8],
        m[6]*n[0] + m[7]*n[3] + m[8]*n[6],
        m[6]*n[1] + m[7]*n[4] + m[8]*n[7],
        m[6]*n[2] + m[7]*n[5] + m[8]*n[8]
    ]

def u_eq_m_times_v_plus_t(u, m, v, t):
    assert len(u) == 3
    assert len(v) == 3
    assert len(t) == 3
    assert len(m) == 9
    if u[0] != m[0]*v[0] + m[1]*v[1] + m[2]*v[2] + t[0]:
        return False
    elif u[1] != m[3]*v[0] + m[4]*v[1] + m[5]*v[2] + t[1]:
        return False
    else:
        return u[2] == m[6]*v[0] + m[7]*v[1] + m[8]*v[2] + t[2]

def mvmult(m, v):
    assert len(m) == 9
    assert len(v) == 3
    return [
        m[0]*v[0] + m[1]*v[1] + m[2]*v[2],
        m[3]*v[0] + m[4]*v[1] + m[5]*v[2],
        m[6]*v[0] + m[7]*v[1] + m[8]*v[2]
    ]

def minverse(m):
    assert len(m) == 9
    mm = np.matrix(m).reshape(3,3)
    mm = np.linalg.inv(mm).reshape(1, 9)
    mm = [int(x) for x in mm.tolist()[0]]
    assert len(mm) == 9
    return mm

def parse_input(path):
    with open(path) as f:
        input = f.read()
    scanners = []
    for index, section in enumerate(input.strip().split("\n\n")):
        beacons = []
        for line in section.split("\n")[1:]:
            beacons.append([int(v) for v in line.strip().split(",")])
        scanners.append(Scanner(index, beacons))
    return scanners

    
def all_rotations():
    """Returns matrices of all possible rotations."""
    units = [[1,0,0], [0,1,0], [0,0,1], [-1,0,0], [0,-1,0], [0,0,-1]]
    combs = [unit for unit in combinations(units, 3)]
    perms = [perm for comb in combs for perm in permutations(comb)]
    rotations = [mmat(perm) for perm in perms]
    #rotations = [rot for rot in rotations if mdet(rot) == 1]
    #assert len(rotations) == 24
    return rotations

Rotations = all_rotations()


class Scanner:

    def __init__(self, index, beacons):
        self.id = index
        self.pos0 = None
        # coord_0 = rot * coord_i + trans
        self.rot = None
        self.trans = None
        self.overlaps = []
        self.beacons = sorted(beacons, key=lambda b: -(b[0]**2 + b[1]**2 + b[2]**2))
        self.checked = {}

    def try_connect(self, scanner):
        """Connect to scanner if there is overlap."""
        # No need to check the same couple of scanner twice
        if scanner.id in self.checked:
            return self.checked[scanner.id]
        self.checked[scanner.id] = False
        scanner.checked[self.id] = False
        # For each couple of beacons and rotation, determine translation
        for local in self.beacons:
            for remote in scanner.beacons:
                for rot in Rotations:
                    trans = vsub(local, mvmult(rot, remote))
                    # Check for 12 matches
                    count = 1
                    for i, loc in enumerate(self.beacons):
                        if loc == local:
                            continue
                        if count + (len(self.beacons) - i) < 12:
                            break
                        for rem in scanner.beacons:
                            if rem == remote or loc == rem:
                                continue
                            if u_eq_m_times_v_plus_t(loc, rot, rem, trans):
                                count += 1
                                if count >= 12:
                                    self.create_overlap(scanner, rot, trans)
                                    self.checked[scanner.id] = True
                                    scanner.checked[self.id] = True
                                    return True
        return False

    def create_overlap(self, scanner, rot, trans):
        # self -> scanner
        self.overlaps.append(Overlap(scanner, rot, trans))
        # scanner -> self
        revrot = minverse(rot)
        revtrans = [-e for e in mvmult(revrot, trans)]
        scanner.overlaps.append(Overlap(self, revrot, revtrans))

    def __repr__(self):
        ol = "\n          - ".join([o.__repr__() for o in self.overlaps])
        if len(ol) != 0:
            ol = "\n        - " + ol
        return f"Scanner<{self.id} beacons:{len(self.beacons)} overlaps:{len(self.overlaps)}{ol}>"


class Overlap:

    def __init__(self, scanner, rot, trans):
        self.scanner = scanner
        # coord_owner = rot * coord_scanner + trans
        self.rot = rot
        self.trans = trans

    def __repr__(self):
        rot = str(self.rot).replace('\n', '').replace(' ', '')
        trans = str(self.trans).replace('\n', '').replace(' ', '')
        return f"Overlap<rot:{rot}, trans:{trans}>"


def connect_all(scanner, rot, trans):
    if scanner.rot is not None:
        return
    scanner.rot = rot
    scanner.trans = trans
    for overlap in scanner.overlaps:
        connect_all(
            overlap.scanner,
            mmult(rot, overlap.rot), 
            vadd(mvmult(rot, overlap.trans), trans)
        )


def main():

    scanners = parse_input("./data/input.txt")

    #
    # Investigation:
    # ==============
    # Those are the value of rot and trans that should be found for scanners[2]
    #trans = [1105,-1205,1229]
    #rot = [-1, 0, 0, 0, 0, 1, 0, 1, 0]
    # scanners[2] connects with scanners[4] with the folloing points (in ref of scanners[0])
    #inter4and2in_ref0 = [[612, -1593, 1893], [496, -1584, 1900], [423, -701, 434], [528, -643, 409], [465, -695, 1988], [408, -1815, 803], [432, -2009, 850], [534, -1912, 768], [527, -524, 1933], [459, -707, 401], [605, -1665, 1952], [456, -540, 1869]]
    # The following rotation connects scanners 2 and 4 (but it has a det of -1 and isn't part of the accepted rotations.)
    # Rotations = [[0,1,0, 1,0,0, 0,0,-1]]

    # Beacons from scanners[2] and scanners[4] that turns out to be the same beacons in ref 0:
    #   b2: [682, -795, 504] b2_0: [423, -701, 434] b4: [-627, -443, -432] b4_0: [423, -701, 434]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [646, -828, 498] b2_0: [459, -707, 401] b4: [-660, -479, -426] b4_0: [459, -707, 401]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [577, -820, 562] b2_0: [528, -643, 409] b4: [-652, -548, -490] b4_0: [528, -643, 409]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [578, 704, 681] b2_0: [527, -524, 1933] b4: [872, -547, -609] b4_0: [527, -524, 1933]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [649, 640, 665] b2_0: [456, -540, 1869] b4: [808, -476, -593] b4_0: [456, -540, 1869]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [640, 759, 510] b2_0: [465, -695, 1988] b4: [927, -485, -438] b4_0: [465, -695, 1988]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [673, -379, -804] b2_0: [432, -2009, 850] b4: [-211, -452, 876] b4_0: [432, -2009, 850]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [697, -426, -610] b2_0: [408, -1815, 803] b4: [-258, -428, 682] b4_0: [408, -1815, 803]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [571, -461, -707] b2_0: [534, -1912, 768] b4: [-293, -554, 779] b4_0: [534, -1912, 768]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [500, 723, -460] b2_0: [605, -1665, 1952] b4: [891, -625, 532] b4_0: [605, -1665, 1952]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [609, 671, -379] b2_0: [496, -1584, 1900] b4: [839, -516, 451] b4_0: [496, -1584, 1900]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
    #   b2: [493, 664, -388] b2_0: [612, -1593, 1893] b4: [832, -632, 460] b4_0: [612, -1593, 1893]
    #     scanner ( 2  - rot: [-1, 0, 0, 0, 0, 1, 0, 1, 0] trans: [1105, -1205, 1229]
    #     scanner ( 4  - rot: [0, -1, 0, 0, 0, -1, 1, 0, 0] trans: [-20, -1133, 1061]
  
    #
    # /Investigation
    #


    couples = [
        [0,7],
        [0,25],
        [1,9],
        [1,12],
        [1,15],
        [1,20],
        [2,3],
        [3,10],
        [3,25],
        [4,28],
        [5,15],
        [5,16],
        [5,20],
        [6,12],
        [6,24],
        [6,34],
        [6,35],
        [7,34],
        [7,35],
        [8,32],
        [10,22],
        [10,29],
        [11,19],
        [12,27],
        [13,14],
        [13,28],
        [14,33],
        [15,26],
        [15,27],
        [16,18],
        [17,19],
        [17,22],
        [17,29],
        [19,28],
        [21,35],
        [22,31],
        [23,24],
        [23,32],
        [23,34],
        [26,33],
        [27,32],
        [27,34],
        [28,33],
        [30,31],
        [31,34]]

    # Connects scanners
    # for a in scanners:
    #     for b in scanners:
    for (i, j) in couples:
        a = scanners[i]
        b = scanners[j]
        print(f"{a.id} <-> {b.id}: ", end='', flush=True)
        if a.id == b.id:
            print("-")
            continue
        if a.try_connect(b):
            print("ok")
        else:
            print("nope")
    connect_all(
        scanners[0],
        rot=[1, 0, 0, 0, 1, 0, 0, 0, 1],
        trans=[0, 0, 0]
    )
    
    # Bring all beacons in s0 coordinate system
    beacons = {}
    for scanner in scanners:
        for beacon in scanner.beacons:
            k = vadd(mvmult(scanner.rot, beacon), scanner.trans)
            beacons.setdefault(repr(k), k)

    print("Part 1 - beacons:", len(beacons))

    maxi = 0
    for a in scanners:
        for b in scanners:
            d = sum([abs(x-y) for (x,y) in zip(a.trans, b.trans)])
            if d > maxi:
                maxi = d

    print("Part 2 - distance:", maxi)


if __name__ == '__main__':
    main()
