import sys
from pathlib import Path
# Add the ../ directory to the Python path                                                                                                   
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc

ages = list(map(int, aoc.read_file_to_string_list("6.test.txt")[0].split(",")))
fish_map = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}

for age in ages:
    if age not in fish_map:
        fish_map[age] = 0
    fish_map[age] += 1

for day in range(257):
    print(f"Day {day}, n={sum(fish_map.values())}")
    if day == 80:
        ANS1 = sum(fish_map.values())
    if day == 256:
        ANS2 = sum(fish_map.values())

    new_fish = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}
    items = fish_map.items()
    for age in range(9):
        num_fish = fish_map[age]
        if age == 0:
            new_fish[6] += num_fish
            new_fish[8] += num_fish
        else:
            new_fish[age - 1] += num_fish
    fish_map = new_fish

print("ans1={}".format(ANS1))  # ans1=352195
print("ans2={}".format(ANS2))  # ans2=1600306001288
