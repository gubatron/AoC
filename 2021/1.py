from aoc import *
import sys
ANS1=0
ANS2=0
depths = readIntList("1.1.txt")
# part 1
prev=sys.maxsize
for d in depths:
    if d > prev:
        ANS1 += 1
    prev = d
# part 2
i=0
prev=None
num_depths=len(depths)
while i < num_depths:
    if i < num_depths-2:
        window = [depths[i], depths[i+1], depths[i+2]]
        if prev is not None and sum(prev) < sum(window):
            ANS2+=1
        prev = window
    i+=1
print("ans1={}".format(ANS1))
print("ans2={}".format(ANS2))
