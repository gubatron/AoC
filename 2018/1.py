# By @gubatron - Nov 10, 2021 (warm up in python for Advent of Code 2021)
import sys
from pathlib import Path
# Add the ../ directory to the Python path
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc
changes = list(map(int, aoc.read_int_list('1.1.txt')))
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
