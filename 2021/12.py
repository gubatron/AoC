import sys
from pathlib import Path
# Add the ../ directory to the Python path
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc

graph = {}
paths = aoc.read_file_to_string_list("12.txt")

for path in paths:
    a, b = path.split('-')
    if a not in graph:
        graph[a] = [b]
    else:
        graph[a].append(b)
    if b not in graph:
        graph[b] = [a]
    else:
        graph[b].append(a)
print(graph)


def countPaths(graph, current, seen):
    if current == 'end':
        return 1
    if current.islower() and current in seen:
        return 0
    seen = [current] + seen
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
    seen = [current] + seen
    count = 0
    for node in graph[current]:
        count += countPaths2(graph, node, seen, repeat)
    return count


ANS1 = countPaths(graph, 'start', [])  # 3410
ANS2 = countPaths2(graph, 'start', [], None)  # 98796

print("ans1={}".format(ANS1))
print("ans2={}".format(ANS2))
