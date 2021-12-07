from math import floor
import statistics
import aoc

crabPositions = list(map(int, aoc.readFileToStringList("7.txt")[0].split(',')))
crabPositions.sort()
print(crabPositions)

median = int(statistics.median(crabPositions))
assert (median in crabPositions)

totalFuelSpent = 0
for crabPos in crabPositions:
    fuel = abs(crabPos - median)
    totalFuelSpent += fuel
    #print(f"Move from {crabPos} to {median}: {fuel} fuel (total={totalFuelSpent})")

ANS1 = totalFuelSpent
print("ans1={}".format(ANS1))  # ans1=364898

totalFuelSpent = 0
optimalPos = floor(statistics.mean(crabPositions))
for crabPos in crabPositions:
    fuel = sum(list(range(1, 1 + abs(crabPos - optimalPos))))
    totalFuelSpent += fuel
    #print(f"Move from {crabPos} to {median}: {fuel} fuel (total={totalFuelSpent})")
ANS2 = totalFuelSpent

print("ans2={}".format(ANS2))  # ans2=104149091
