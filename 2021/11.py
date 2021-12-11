from functools import reduce
import aoc

octos = aoc.readIntMatrix('11.txt')


def increaseEnergyLevels(octos):
    for r in range(len(octos)):
        octos[r] = list(map(lambda x: x + 1, octos[r]))


print("Before any steps:")
aoc.printMatrix(octos)
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
                shockwave = aoc.getSurroundingCoords(r, c, old_octos,
                                                     True) + shockwave

    while len(shockwave) > 0:
        x, y = shockwave.pop(0)
        if old_octos[x][y] != 10:
            old_octos[x][y] += 1
            if old_octos[x][y] == 10:
                shockwave = aoc.getSurroundingCoords(x, y, old_octos,
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
    aoc.printMatrix(octos)
    print()

    if ANS2 != 0:
        break

aoc.printMatrix(octos)
print("ans1={}".format(ANS1))  # 1601
print("ans2={}".format(ANS2))  # 368
