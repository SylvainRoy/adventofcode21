#!/usr/bin/env python

from functools import reduce

class Packet:

    def __init__(self, version, type_id):
        self.version = version
        self.type_id = type_id

    def sum_version(self):
        return self.version


class Literal (Packet):

    def __init__(self, version, type_id):
        super().__init__(version, type_id)
        self.value = None

    def __str__(self):
        return f"Literal[{self.version}/{self.type_id} value:{self.value}]"

    def __repr__(self):
        return self.__str__()

    def tree_str(self, indent=""):
        return indent + str(self)

    def decode(self, data):
        more = True
        self.value = 0
        i = 0
        while more:
            more = (data[i] == '1')
            self.value = 16 * self.value + int(data[i+1:i+5], 2)
            i += 5
        return i

    def eval(self):
        return self.value


class Operator (Packet): 

    def __init__(self, version, type_id):
        super().__init__(version, type_id)
        self.packets = []

    def __str__(self):
        return f"Operator[{self.version}/{self.type_id} packets: {len(self.packets)}]"

    def __repr__(self):
        return self.__str__()

    def tree_str(self, indent=""):
        out = indent
        out += str(self)
        for sub in self.packets:
            out += "\n" + sub.tree_str(indent + "  ")
        return out

    def decode(self, data):
        length_type_id = int(data[0], 2)
        if length_type_id == 0:
            total_length = int(data[1:16], 2)
            index = 16
            while index < 16 + total_length:
                subpacket, readp = decode(data[index:])
                index += readp
                self.packets.append(subpacket)
        else:
            num_packet = int(data[1:12], 2)
            index = 12
            for i in range(num_packet):
                subpacket, readp = decode(data[index:])
                index += readp
                self.packets.append(subpacket)
        return index

    def sum_version(self):
        return self.version + sum(sub.sum_version() for sub in self.packets)

    def eval(self):
        if self.type_id == 0: # sum
            return sum(p.eval() for p in self.packets)
        elif self.type_id == 1: # product
            return reduce(
                lambda x,y: x*y,
                (p.eval() for p in self.packets)
            )
        elif self.type_id == 2: # min
            return reduce(
                min,
                (p.eval() for p in self.packets)
            )
        elif self.type_id == 3: # max
            return reduce(
                max,
                (p.eval() for p in self.packets)
            )
        elif self.type_id == 5: # greater than
            if self.packets[0].eval() > self.packets[1].eval():
                return 1
            else: 
                return 0
        elif self.type_id == 6: # less than
            if self.packets[0].eval() < self.packets[1].eval():
                return 1
            else: 
                return 0
        elif self.type_id == 7: # equal to
            if self.packets[0].eval() == self.packets[1].eval():
                return 1
            else: 
                return 0


def decode(data):
    version = int(data[:3], 2)
    type_id = int(data[3:6], 2)
    if type_id == 4: # literal
        packet = Literal(version, type_id)
    else:            # operator
        packet = Operator(version, type_id)
    index = 6 + packet.decode(data[6:])
    return packet, index


def binary(str):
    out = ""
    for car in str:
        out += format(int(car, 16), '04b')
    return out


def main():
    with open("./data/input.txt") as f:
        hexstr = f.read().strip()      
    binstr = binary(hexstr)

    # tempo:
    # print("test 1: ", decode(binary("D2FE28")))
    # print("test 2a: ", decode("11010001010"))
    # print("test 2: ", decode(binary("38006F45291200"))[0])
    # p, i = decode(binary("EE00D40C823060"))
    # print("test 3: ", p.tree_str())

    #
    # Part 1 and 2
    #
    p, i = decode(binstr)
    print("Part 1 - sum versions: ", p.sum_version())
    print("Part 2 - eval: ", p.eval())

if __name__ == "__main__":
    main()
