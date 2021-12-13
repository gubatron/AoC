import aoc

raw_data = aoc.readFileToStringList('13.txt')


def loadData(raw_data):
    dots = set()
    fold_instructions = []
    emptyLineFound = False
    for line in raw_data:
        if len(line) == 0:
            emptyLineFound = True
        elif not emptyLineFound:
            dots.add(tuple(map(int, line.split(','))))
        else:
            fold_instructions.append(line.split('fold along ')[1].split('='))
    return dots, fold_instructions


def getMaxXY(dots):
    max_x = 0
    max_y = 0
    for (x, y) in dots:
        if x > max_x:
            max_x = x
        if y > max_y:
            max_y = y
    return max_x, max_y


def printMatrix(dots):
    max_x, max_y = getMaxXY(dots)
    # y = rows
    # x = cols
    for y in range(max_y + 1):
        row = f"{y} "
        for x in range(max_x + 1):
            if (x, y) in dots:
                row += ('#')
            else:
                row += ('.')
        print(row)
    print()


def foldPageAtX(dots, X):
    new_dots = set()
    for (x, y) in dots:
        if x > X:
            new_dots.add((2*X-x, y))
        else:
            new_dots.add((x, y))
    return new_dots


def foldPageAtY(dots, Y):
    new_dots = set()
    for (x, y) in dots:
        if y >= Y:
            new_dots.add((x, 2*Y-y))
        else:
            new_dots.add((x, y))
    return new_dots


dots, fold_instructions = loadData(raw_data)
visible_at_step = [len(dots)]

for inst in fold_instructions:
    if inst[0] == 'x':
        dots = foldPageAtX(dots, int(inst[1]))
    else:
        dots = foldPageAtY(dots, int(inst[1]))
    visible_at_step.append(len(dots))

printMatrix(dots)  # step away and read ZKAUCFUC

ANS1 = visible_at_step[1]
ANS2 = 0

print("ans1={}".format(ANS1))  # 731
print("ans2={}".format(ANS2))
