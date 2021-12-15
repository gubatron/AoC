from collections import Counter
import aoc

data = aoc.readFileToStringList('14.txt')

template = data[0]

rules = {}
for r in data[2:]:
    a, b = r.split(' -> ')
    rules[a] = b


def initDB(template):
    counter = Counter()
    for i in range(len(template) - 1):
        pair = template[i:i + 2]
        counter[pair] += 1
    return counter


def walk(counter, rules, steps=10):
    for _ in range(steps):
        new_counter = Counter()
        for k, count in counter.items():
            if k in rules:
                pairA = k[0] + rules[k]
                pairB = rules[k] + k[1]
                new_counter[pairA] += count
                new_counter[pairB] += count
        counter = new_counter
    return counter


def solve(template, steps=10):
    initialPolymer = template
    counter = initDB(template)
    counter = walk(counter, rules, steps)
    atomStats = Counter()
    # letters on the outside don't change
    atomStats[initialPolymer[0]] = 1
    atomStats[initialPolymer[-1]] = 1
    for pair, count in counter.items():
        # count each letter
        atomStats[pair[0]] += count
        atomStats[pair[1]] += count
    sortedAtoms = atomStats.most_common()
    return (sortedAtoms[0][1] - sortedAtoms[-1][1]) // 2


ANS1 = solve(template, 10)
ANS2 = solve(template, 40)

print("ans1={}".format(ANS1))  # 2915
print("ans2={}".format(ANS2))  # 3353146900153
