from collections import Counter
import aoc

data = aoc.readFileToStringList('14.txt')

template = data[0]

rules = {}
for r in data[2:]:
    a, b = r.split(' -> ')
    rules[a] = b


def initDB(template):
    database = Counter()
    for i in range(len(template) - 1):
        pair = template[i:i + 2]
        database[pair] += 1
    return database


def walk(database, rules, steps=10):
    for step in range(steps):
        new_database = Counter()
        for k, count in database.items():
            if k in rules:
                pairA = k[0] + rules[k]
                pairB = rules[k] + k[1]
                new_database[pairA] += count
                new_database[pairB] += count
        database = new_database
    return database


def solve(template, steps=10):
    initialPolymer = template
    database = initDB(template)
    database = walk(database, rules, steps)
    atomStats = Counter()
    # letters on the outside don't change
    atomStats[initialPolymer[0]] = 1
    atomStats[initialPolymer[-1]] = 1
    for pair, count in database.items():
        # count each letter
        atomStats[pair[0]] += count
        atomStats[pair[1]] += count
    sortedAtoms = atomStats.most_common()
    return (sortedAtoms[0][1] - sortedAtoms[-1][1]) // 2


ANS1 = solve(template, 10)
ANS2 = solve(template, 40)

print("ans1={}".format(ANS1))  # 2915
print("ans2={}".format(ANS2))  # 3353146900153
