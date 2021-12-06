import aoc

ages = list(map(int, aoc.readFileToStringList("6.1.txt")[0].split(",")))
fish_map = {}

for age in ages:
    if age not in fish_map:
        fish_map[age] = 0
    fish_map[age] += 1

for day in range(257):
    print(f"Day {day}, n={sum(fish_map.values())}")
    if day == 80:
        ANS1 = sum(fish_map.values())
        print(ANS1)
    if day == 256:
        ANS2 = sum(fish_map.values())

    new_fish = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}
    for age, num_fish in fish_map.items():
        if age == 0:
            new_fish[6] += num_fish
            new_fish[8] += num_fish
        else:
            new_fish[age - 1] += num_fish
    fish_map = new_fish

print("ans1={}".format(ANS1))  # ans1=352195
print("ans2={}".format(ANS2))  # ans2=1600306001288