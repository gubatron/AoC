from collections import defaultdict
import aoc

graph = defaultdict(list)
paths = aoc.readFileToStringList("12.txt")

for path in paths:
    a, b = path.split('-')
    graph[a].append(b)
    graph[b].append(a)

print(graph)


def countPaths(graph, current, seen):
    if current == 'end':
        return 1
    if current.islower() and current in seen:
        return 0
    seen = seen | {current}
    count = 0
    for node in graph[current]:
        count += countPaths(graph, node, seen)
    return count


def countPaths2(graph, current, seen, repeat):
    if current == 'start' and seen:
        return 0
    if current == 'end':
        return 1
    if current.islower() and current in seen:
        if repeat is None:
            repeat = current
        else:
            return 0
    seen = seen | {current}
    count = 0
    for node in graph[current]:
        count += countPaths2(graph, node, seen, repeat)
    return count


ANS1 = countPaths(graph, 'start', set())  # 3410
ANS2 = countPaths2(graph, 'start', set(), None)  # 98796

print("ans1={}".format(ANS1))
print("ans2={}".format(ANS2))