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

def generatePointsBetween(a: Point, b: Point, doDiagonals: bool) -> set[Point]:
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
    if doDiagonals and abs(a.x - b.x) == abs(a.y - b.y):  # diagonal segment
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


def genPoints(a:Point,b:Point,points:dict,considerDiagonals:bool):
    midPoints = generatePointsBetween(a, b, considerDiagonals)
    if midPoints != None:
        updatePoints(points, a)
        updatePoints(points, b)
        if len(midPoints) > 0:
            list(map(lambda p: updatePoints(points, p), midPoints))

# PART 1.
points = {}
points2 = {}
def f(seg):
      a = Point(seg[START])
      b = Point(seg[END])
      genPoints(a,b,points,False)
      genPoints(a,b,points2,True)
list(map(f,segments))

ANS1 = len({k: v for k, v in points.items() if v > 1})
print("ans1={}".format(ANS1))  # ans1=7085
ANS2 = len({k: v for k, v in points2.items() if v > 1})
print("ans2={}".format(ANS2)) # ans2=20271