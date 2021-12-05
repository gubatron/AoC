import aoc
data = aoc.readFileToStringList("5.1.txt")
segments = list(
    map(
        lambda line: list(
            map(lambda pStr: list(map(int, pStr.split(','))), line.split(
                ' -> '))), data))
START = 0
END = 1
class Point:
    def __init__(self, p: list[int]) -> None:
        self.x = p[0]
        self.y = p[1]

    def __hash__(self):
        return hash(self.__repr__())

    def __repr__(self) -> str:
        return "({},{})".format(self.x, self.y)

    def __str__(self) -> str:
        return self.__repr__()

def updatePoints(points: dict[str, int], p: Point) -> None:
    if str(p) in points:
        points[str(p)] += 1
    else:
        points[str(p)] = 1

def generatePointsBetween(a: Point, b: Point) -> set[Point]:
    midPoints = []
    c = Point([a.x, a.y])
    if a.x == b.x:  # vertical segment.
        delta = -1
        if b.y > a.y:
            delta = 1
        while c.y != b.y - delta:
            c.y += delta
            midPoints.append(Point([c.x, c.y]))
        return midPoints
    elif a.y == b.y:  # horizontal segment.
        delta = -1
        if b.x > a.x:
            delta = 1
        while c.x != b.x - delta:
            c.x += delta
            midPoints.append(Point([c.x, c.y]))
        return midPoints
    return None

# PART 1.
points = {}
for seg in segments:
    a = Point(seg[START])
    b = Point(seg[END])
    midPoints = generatePointsBetween(a, b)
    if midPoints != None:
        updatePoints(points, a)
        updatePoints(points, b)
        if len(midPoints) > 0:
            list(map(lambda p: updatePoints(points, p), midPoints))

ANS1 = len({k: v for k, v in points.items() if v > 1})
print("ans1={}".format(ANS1))  # ans1=7085

# PART 2.
def generatePointsBetween2(a: Point, b: Point) -> set[Point]:
    midPoints = []
    c = Point([a.x, a.y])
    if a.x == b.x:  # vertical segment.
        delta = -1
        if b.y > a.y:
            delta = 1
        while c.y != b.y - delta:
            c.y += delta
            midPoints.append(Point([c.x, c.y]))
        return midPoints
    if a.y == b.y:  # horizontal segment.
        delta = -1
        if b.x > a.x:
            delta = 1
        while c.x != b.x - delta:
            c.x += delta
            midPoints.append(Point([c.x, c.y]))
        return midPoints
    if abs(a.x - b.x) == abs(a.y - b.y):  # diagonal segment
        # lets always go from a to b
        deltaX = -1
        deltaY = -1
        if b.x > a.x:
            deltaX = 1
        if b.y > a.y:
            deltaY = 1
        while c.x != b.x - deltaX and c.y != b.y - deltaY:
            c.x += deltaX
            c.y += deltaY
            midPoints.append(Point([c.x, c.y]))
        return midPoints
    return None

points = {}
for seg in segments:
    a = Point(seg[START])
    b = Point(seg[END])
    midPoints = generatePointsBetween2(a, b)
    if len(midPoints) > 0:
        list(map(lambda p: updatePoints(points, p), [a] + midPoints + [b]))
    elif midPoints != None:
        updatePoints(points, a)
        updatePoints(points, b)

ANS2 = len({k: v for k, v in points.items() if v > 1})
print("ans2={}".format(ANS2)) # ans2=20271