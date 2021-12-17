def readFileToStringList(path, stripped=True):
    fp = open(path, mode='r', buffering=4096)
    result = fp.readlines()
    if stripped:
        result = [s.strip() for s in result]
    fp.close()
    return result


def readStringsBySeparator(path, separator, stripped=True):
    stringList = readFileToStringList(path, stripped)
    bigString = ''.join(stringList)
    if not stripped:
        return bigString.split(separator)
    return [s.strip() for s in bigString.split(separator)]


def readIntList(path, stripped=True):
    stringList = readFileToStringList(path, stripped)
    return list(map(int, stringList))


def readIntMatrix(path, stripped=True):
    data = readFileToStringList(path)
    G = []
    for l in list(map(list, data)):
        G.append(list(map(int, l)))
    return G


def getSurroundingCoords(r, c, graph, includeDiagonals=False):
    surrounding = []
    deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    if includeDiagonals:
        deltas += [(-1, -1), (-1, 1), (1, -1), (1, 1)]
    rows, cols = len(graph), len(graph[0])
    for (dx, dy) in deltas:
        if r + dx >= 0 and r + dx < rows and c + dy >= 0 and c + dy < cols:
            surrounding.append((r + dx, c + dy))
    return surrounding


def printMatrix(G, message=None):
    if message != None:
        print(message)
    for r in G:
        print(r)
    print()


def DFS(GRAPH, source, target):
    'Use a STACK to search.'
    stack = [source]
    visited = []

    while len(stack) > 0:
        x = stack.pop(0)

        if x == target:
            visited.append(x)
            return visited
        elif x not in visited:
            visited = visited + [x]
            if GRAPH[x] is not None:
                'add nodes at the top of the stack'
                stack = GRAPH[x] + stack

    return visited


def BFS(source, target, GRAPH):
    'Use a QUEUE to search.'
    queue = [source]
    visited = []

    while len(queue) > 0:
        x = queue.pop(0)

        if x == target:
            visited.append(x)
            return visited
        elif x not in visited:
            visited = visited + [x]
            if GRAPH[x] is not None:
                'add nodes at the END of the queue'
                queue = queue + GRAPH[x]

    return visited


#
# GRAPH is a dict of lists
# The Keys are the Node Ids or Coordinates as a tuple (x,y)
# The values are a dict of adjacentNode IDs (or coord tuples) and their distances to the key
#   key   -> adjacent node coordinates (x,y) or identifier
#   value -> distance or cost
# costs cannot be negative, Dijkstra fails for negative costs
# { (fooNodeCoordinateX, fooNodeCoordinateY): {(adjacentX, adjacentY): adjacentCost, .. } }
#
def DIJKSTRA(source, end, GRAPH, infinity=10**100):
    distances = {}
    sourceAdjacents = GRAPH[source].keys()
    for node in GRAPH:
        if node in distances:
            continue
        if node == source:
            distances[source] = 0
        elif node in sourceAdjacents:
            distances[node] = GRAPH[source][node]
        else:
            distances[node] = infinity
    queue = [source]
    visited = []

    while len(queue) > 0:
        # find nearest non visited node (TODO: use a priority queue for this)
        nearestNode = None
        if len(queue) == 1 and not (queue[0] in visited):
            nearestNode = queue[0]
        else:
            for node in queue:
                # scan the queue for closest one
                if nearestNode is None:
                    nearestNode = node
                elif distances[node] < distances[nearestNode]:
                    nearestNode = node
        # remove nearest node from the queue
        if nearestNode not in visited and not nearestNode is None:
            visited.append(nearestNode)
        if nearestNode in queue:
            queue.remove(nearestNode)

        # get the adjacent nodes to the neaest we have not visited and update their distances
        adjacentNodes = GRAPH[nearestNode]

        for adjacentNodeId, cost in adjacentNodes.items():
            # adjust distance from our starting node (minDistanceNode) to its adjacents
            # optimize the distance
            if distances[nearestNode] + cost < distances[adjacentNodeId]:
                distances[adjacentNodeId] = distances[nearestNode] + cost
            if adjacentNodeId not in queue and adjacentNodeId not in visited:
                queue.append(adjacentNodeId)
    return visited, distances


def testDijkstra():
    graph1 = {
        (1, 1): {
            (2, 2): 2,
            (3, 3): 4
        },
        (2, 2): {
            (3, 3): 1,
            (4, 4): 7
        },
        (3, 3): {
            (5, 5): 3
        },
        (4, 4): {
            (6, 6): 1
        },
        (5, 5): {
            (4, 4): 2,
            (6, 6): 5
        },
        (6, 6): {
            (6, 6): 0
        }
    }

    graph2 = {
        1: {
            2: 50,
            3: 45,
            4: 10
        },
        2: {
            3: 10,
            4: 15
        },
        3: {
            5: 30
        },
        4: {
            5: 15
        },
        5: {
            2: 20,
            3: 35
        }
    }

    #visited, distances = DIJKSTRA((1, 1), (6, 6), graph1)
    #print(f'visited:{visited}\ndistances:{distances}')
    visited, distances = DIJKSTRA(1, 5, graph2)
    print(f'visited:{visited}\ndistances:{distances}')


if __name__ == '__main__':
    #print(readStringsBySeparator('strings_by_sep_test.txt', '-'))
    #print(readIntList('int_list.txt'))
    testDijkstra()
    pass
