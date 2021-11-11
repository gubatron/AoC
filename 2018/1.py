# By @gubatron - Nov 10, 2021 (warm up in python for Advent of Code 2021)
from aoc import *
changes = list(map(int, readIntList('1.1.txt')))
ans1 = sum(changes)

found = False
ans2 = 0
seen = {0}
frequency = 0
loops = 0
while not found:
  for change in changes:
    frequency += change
    if frequency not in seen:
        seen.add(frequency)
    else:
        ans2 = frequency
        found = True
        print("found repeated frequency: {} on loop {}".format(frequency, loops))
        break
  loops+=1

print("answer 1: {}".format(ans1)) # 454
print("answer 2: {}".format(ans2)) # 566