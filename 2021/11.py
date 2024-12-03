from functools import reduce
import sys
from pathlib import Path
# Add the ../ directory to the Python path
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc

octos = aoc.read_int_matrix('11.txt')


def increaseEnergyLevels(octos):
    for r in range(len(octos)):
        octos[r] = list(map(lambda x: x + 1, octos[r]))


print("Before any steps:")
aoc.print_matrix(octos)
print()
ANS1 = 0
ANS2 = 0
steps = 0
while True:
    old_octos = octos[:]
    increaseEnergyLevels(old_octos)

    # find out who flashed around, set them to -1
    shockwave = []
    for r in range(len(old_octos)):
        for c in range(len(old_octos[0])):
            if old_octos[r][c] == 10:
                shockwave = aoc.get_surrounding_coords(r, c, old_octos,
                                                     True) + shockwave

    while len(shockwave) > 0:
        x, y = shockwave.pop(0)
        if old_octos[x][y] != 10:
            old_octos[x][y] += 1
            if old_octos[x][y] == 10:
                shockwave = aoc.get_surrounding_coords(x, y, old_octos,
                                                     True) + shockwave

    # reset to zero everybody that flashed and count flashes in this step
    flashesThisStep = 0
    for r in range(len(old_octos)):
        for c in range(len(old_octos[0])):
            if old_octos[r][c] == 10:
                old_octos[r][c] = 0
                flashesThisStep += 1
                if steps < 100:
                    ANS1 += 1

    octos = old_octos[:]
    steps += 1
    if flashesThisStep == 100:
        ANS2 = steps

    print(f"step {steps}, flashes {ANS1}")
    aoc.print_matrix(octos)
    print()

    if ANS2 != 0:
        break

aoc.print_matrix(octos)
print("ans1={}".format(ANS1))  # 1601
print("ans2={}".format(ANS2))  # 368
