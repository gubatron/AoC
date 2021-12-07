from math import floor
import statistics
import aoc
crabPositions = list(map(int, aoc.readFileToStringList("7.txt")[0].split(',')))
crabPositions.sort()
median = int(statistics.median(crabPositions))
mean = floor(statistics.mean(crabPositions))
totalFuel1 = 0
totalFuel2 = 0
for crabPos in crabPositions:
    totalFuel1 += abs(crabPos - median)
    totalFuel2 += sum(range(1, 1 + abs(crabPos - mean)))
ANS1 = totalFuel1
ANS2 = totalFuel2
print("ans1={}".format(ANS1))  # ans1=364898
print("ans2={}".format(ANS2))  # ans2=104149091