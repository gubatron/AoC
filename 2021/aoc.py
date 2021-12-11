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


if __name__ == '__main__':
    print(readStringsBySeparator('strings_by_sep_test.txt', '-'))
    print(readIntList('int_list.txt'))