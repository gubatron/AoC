import sys
from pathlib import Path
# Add the ../ directory to the Python path
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc

instructions = aoc.read_file_to_string_list("2.1.txt")
horizontal = 0
depth = 0
aim = 0

#instructions = ["forward 5","down 5","forward 8","up 3","down 8","forward 2"]


def process_instruction(inst):
    global horizontal, depth
    action, step = (inst.split())
    step = int(step)
    if action == 'forward':
        horizontal += step
    if action == 'up':
        depth -= step
    if action == 'down':
        depth += step


list(map(process_instruction, instructions))

ans1 = depth * horizontal

horizontal = 0
depth = 0
aim = 0


def process_instruction2(inst):
    global horizontal, depth, aim
    action, step = (inst.split())
    step = int(step)
    if action == 'forward':
        horizontal += step
        depth += aim * step
    if action == 'up':
        aim -= step
    if action == 'down':
        aim += step


list(map(process_instruction2, instructions))

ans2 = depth * horizontal

print("ans1={}".format(ans1))  # ans1=1868935
print("ans2={}".format(ans2))  # ans2=1965970888
