from math import floor
from statistics import median, mean
import sys
from pathlib import Path
# Add the ../ directory to the Python path                                                                                                   
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc
crabPositions = list(map(int, aoc.read_file_to_string_list("7.txt")[0].split(',')))
median,mean = int(median(crabPositions)), floor(mean(crabPositions))
ANS1 = ANS2 = 0
for crabPos in crabPositions:
    ANS1 += abs(crabPos - median)
    ANS2 += sum(range(1, 1 + abs(crabPos - mean)))
print("ans1={}".format(ANS1))  # ans1=364898
print("ans2={}".format(ANS2))  # ans2=104149091
