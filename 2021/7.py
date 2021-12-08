from math import floor
from statistics import median, mean
import aoc
crabPositions = list(map(int, aoc.readFileToStringList("7.txt")[0].split(',')))
median,mean = int(median(crabPositions)), floor(mean(crabPositions))
ANS1 = ANS2 = 0
for crabPos in crabPositions:
    ANS1 += abs(crabPos - median)
    ANS2 += sum(range(1, 1 + abs(crabPos - mean)))
print("ans1={}".format(ANS1))  # ans1=364898
print("ans2={}".format(ANS2))  # ans2=104149091