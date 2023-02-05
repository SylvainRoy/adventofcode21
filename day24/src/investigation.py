
#%%
import collections
import itertools
from collections import defaultdict

#%%
# This shows that the program is composed of 14 18-line sub programs.
# Those sub programs are identical with the exception of lines:
# 4: div x a    with a in [1, 26]
# 5: add x b    with b in ['-1', '-13', '-2', '-7', '-8', '10', '11', '12', '13', '14', '15']
# 15: add y c   with c in ['1', '12', '13', '14', '15', '2', '5', '6', '7', '8']

with open("./data/input.txt") as f:
    program = [l.strip() for l in f.readlines()]

setlines = defaultdict(lambda: set())
for line in range(18):
    for sub in range(len(program)//18):
        setlines[line].add(program[18*sub+line])

for line in range(18):
    if len(setlines[line]) != 1:
        print(f"{line}: ", end="")
        for l in setlines[line]:
            print(f"{l} / ", end="")
        print()

sorted([e.split(" ")[2] for e in setlines[15]])


#%%
# Here are the 14 3-uplets of paramters of the sub (that I'll call a, b and c)

abc = []
for sub in range(0, len(program), 18):
    abc.append([int(l.split(" ")[2]) for l in [program[sub+4], program[sub+5], program[sub+15]]])
abc


#%%
# Let's now work on this sub function.

# inp w
# mul x 0
# add x z
# mod x 26
# div z a
# add x b
# eql x w
# eql x 0
# mul y 0
# add y 25
# mul y x
# add y 1
# mul z y
# mul y 0
# add y w
# add y c
# mul y x
# add z y

def fx(z, w, a, b, c):
    x = z % 26
    z //= a
    x += b
    if x == w:
        x = 1
    else:
        x = 0
    if x == 0:
        x = 1
    else:
        x = 0
    y = (25 * x) + 1
    z *= y
    y = (w + c) * x
    z += y

def fx(z, w, a, b, c):
    x = (z % 26) + b
    z //= a
    if x != w:
        x = 1
    else:
        x = 0
    z *= (25 * x) + 1
    z += (w + c) * x

#%%
def f(z, w, a, b, c):
    z1 = z // a
    if (z % 26) + b != w:
        z1 = 26*z1 + w + c
    return z1
#%%


# 'z' can take two values: 1 and 26

def fx(z, w, a, b, c):
    if a == 1:
        z1 = z
        if (z % 26) + b != w:
            z1 = (26 * z1) + w + c
    else: # a == 26
        z1 = z // 26
        if (z % 26) + b != w:
            z1 = (26 * z1) + w + c
    return z1


# 'b' > 10 when 'a' == 1. Then, the condition (z % 26) + b != w is always true.

def fx(z, w, a, b, c):
    if a == 1:
        z1 = (26 * z) + w + c      # in base 26, add digit 'w+c' to right of number 'z' (this is OP)
    else: # a == 26
        z1 //= 26                  # in base 26, remove last digit (This is the reverse of OP)
        if (z % 26) != w - b:      #    we won't want that to be executed!
            z1 = (26 * z1) + w + c
    return z1

# The two operations defined by a==1 and a==26 are the opposite of each
# other IF when a==26:
#   (z % 26) + b == w
#   z % 26 == w - b   <==> the last digit (base 26) of z is w-b

# So...

# a==1 pushes a new base-26 digit, equal to w+c, on the right of z.
# a==26 removes a base-26 digit on the right of z IF this digit is 'w-b'

# Now, using the program:

# push w[0]+5
# push w[1]+5
# push w[2]+1
# push w[3]+15
# push w[4]+2
# pop if digits[-1] == w[5] - (-1)
# push w[6]+5
# pop if digits[-1] == w[7] - (-8)
# pop if digits[-1] == w[8] - (-7)
# pop if digits[-1] == w[9] - (-8)
# push w[10]+7
# pop if digits[-1] == w[11] - (-2)
# pop if digits[-1] == w[12] - (-2)
# pop if digits[-1] == w[13] - (-13)

# push w[0]+5                                   |
# push w[1]+5                                 | |
# push w[2]+1                             |   | |
# push w[3]+15                          | |   | |
# push w[4]+2                       |   | |   | |
# pop if w[4] + 2 == w[5] - (-1)    |   | |   | | 
# push w[6]+5                         | | |   | |
# pop if w[6] + 5 == w[7] - (-8)      | | |   | | 
# pop if w[3] + 15 == w[8] - (-7)       | |   | | 
# pop if w[2] + 1 == w[9] - (-8)          |   | | 
# push w[10]+7                              | | |
# pop if w[10] + 7 == w[11] - (-2)          | | | 
# pop if w[1] + 5 == w[12] - (-2)             | | 
# pop if w[0] + 5 == w[13] - (-13)              |

# w[4] + 2 == w[5] + 1
# w[6] + 5 == w[7] + 8
# w[3] + 15 == w[8] + 7
# w[2] + 1 == w[9] + 8
# w[10] + 7 == w[11] + 2
# w[1] + 5 == w[12] + 2
# w[0] + 5 == w[13] + 13

# w[5] = w[4] + 1
# w[7] = w[6] - 3
# w[8] = w[3] + 8
# w[9] = w[2] - 7
# w[11] = w[10] + 5
# w[12] = w[1] + 3
# w[13] = w[0] - 8


#%%
def numberToBase(n, b):
    if n == 0:
        return [0]
    digits = []
    while n:
        digits.append(int(n % b))
        n //= b
    return digits[::-1]

def status(i, z):
    print(f"{i}: {z} = {numberToBase(z, 26)}")

#%%
#    0 1 2 3 4  5 6 7 8 9 0 1 2 3 
w = [1,9,9,0,9,14,9,7,3,6,9,9,9,9]
w[5] = w[4] + 1
w[7] = w[6] - 3
w[8] = w[3] + 8
w[9] = w[2] - 7
w[11] = w[10] + 5
w[12] = w[1] + 3
w[13] = w[0] - 8
w

#%%
z = 0
i = 0
z = f(0, w[i], *abc[i]) # [1, 11, *5],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [1, 13, *5],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [1, 12, *1],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [1, 15, *15],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [1, 10, *3],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [26, *-1, 2],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [1, 14, *5],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [26, *-8, 8],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [26, *-7, 14],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [26, *-8, 12],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [1, 11, *7],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [26, *-2, 14],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [26, *-2, 13],
i += 1
status(i, z)
z = f(z, w[i], *abc[i]) # [26, *-13, 6],
i += 1
status(i, z)



# %%
# Biggest number:
w0 = 9
w1 = 6
w2 = 9
w3 = 1
w4 = 8
w6 = 9
w10 = 4
w = [w0,w1,w2,w3,w4,w4+1,w6,w6-3,w3+8,w2-7,w10,w10+5,w1+3,w0-8]
print("Part 1 - biggest number:", "".join([str(v) for v in w]))

# %%
#Smallest number:
w0 = 9
w1 = 1
w2 = 8
w3 = 1
w4 = 1
w6 = 4
w10 = 1
w = [w0,w1,w2,w3,w4,w4+1,w6,w6-3,w3+8,w2-7,w10,w10+5,w1+3,w0-8]
print("Part 2 - smallest number:", "".join([str(v) for v in w]))

# %%
