import sys
from pathlib import Path
# Add the ../ directory to the Python path
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc

def initializeGraph(matrix):
    rows = len(matrix)
    cols = len(matrix[0])
    graph = {}

    for row in range(rows):
        for col in range(cols):
            neighs = aoc.get_surrounding_coords(row, col, matrix)
            adjacents_risks = {}
            for neighbor in neighs:
                adjacents_risks[neighbor] = matrix[neighbor[0]][neighbor[1]]
            graph[(row, col)] = adjacents_risks
    return graph

# part 1
matrix = aoc.read_int_matrix("15.txt")
graph = initializeGraph(matrix)
start, end = (0,0), (len(matrix) - 1, len(matrix[0]) - 1)
visited, distances = aoc.DIJKSTRA(start, end, graph)
ANS1 = distances[end]
print("ans1={}".format(ANS1)) # 363

# part 2
def wrapIncrementRow(row, limit=10):
    return list(map(lambda x  : 1 if ((x + 1) % limit == 0) else ((x + 1) % limit), row))

def wrapIncrementMatrix(matrix, limit=10):
    return list(map(wrapIncrementRow, matrix))

# found this to be the hardest part to think about for some reason, after sleeping and dreaming about it
def growMatrix(matrix, limit=10):
    big_matrix = []
    # we add horizontally to make a big row, out of each row and increment it
    for n in range(5):
        for i in range(len(matrix)):
            row = []
            current_mini_row = None
            for j in range(5):
                if current_mini_row == None:
                    current_mini_row = matrix[i]
                row.extend(current_mini_row)
                current_mini_row = wrapIncrementRow(current_mini_row,limit)
            big_matrix.append(row)
        matrix = wrapIncrementMatrix(matrix)

    return big_matrix

bigMatrix = growMatrix(matrix, 10)
bigGraph = initializeGraph(bigMatrix)
start, end = (0,0), (len(bigMatrix) - 1, len(bigMatrix[0]) - 1)
visited, distances = aoc.DIJKSTRA(start, end, bigGraph)
ANS2 = distances[end]

print("ans2={}".format(ANS2)) # 2835
