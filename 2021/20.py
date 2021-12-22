import aoc


def loadImageDict(imgData):
    imgDict = {}
    row = 0
    for line in imgData:
        column = 0
        for c in line:
            imgDict[(column, row)] = c
            column += 1
        row += 1
    return imgDict


def getSurroundingPixels(c, r, imgDict, defaultChar='.'):
    pixels = ''
    deltas = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1),
              (0, 1), (1, 1)]

    for delta in deltas:
        pixCol = c + delta[0]
        pixRow = r + delta[1]
        if (pixCol, pixRow) in imgDict:
            pixel = imgDict[(pixCol, pixRow)]
            pixels += pixel
        else:
            pixels += defaultChar
    return pixels


def pixelsToDecimal(pixels):
    return int(pixels.replace('#', '1').replace('.', '0'), 2)


def getMinMaxCoords(imgDict):
    minCol = minRow = 10 * 100
    maxCol = maxRow = -10 * 100
    for coord in imgDict.keys():
        if coord[0] < minCol:
            minCol = coord[0]
        if coord[0] > maxCol:
            maxCol = coord[0]
        if coord[1] < minRow:
            minRow = coord[1]
        if coord[1] > maxRow:
            maxRow = coord[1]
    return (minCol, maxCol, minRow, maxRow)


def padImage(imgDict, padding=5, defaultChar='.'):
    minCol, maxCol, minRow, maxRow = getMinMaxCoords(imgDict)
    for row in range(minRow - padding, maxRow + padding + 1):
        for col in range(minCol - padding, maxCol + padding + 1):
            if (col, row) not in imgDict:
                imgDict[(col, row)] = defaultChar


def padImageAsNeeded(imgDict, defaultChar):
    while (defaultChar == '.' and not allBordersDark(imgDict)
           or (defaultChar == '#' and not allBordersLit(imgDict))):
        padImage(imgDict, 1, defaultChar)


def enhanceImage(imgDict, algoStr, defaultChar):
    padImageAsNeeded(imgDict, defaultChar)
    minCol, maxCol, minRow, maxRow = getMinMaxCoords(imgDict)
    changes = {}
    for row in range(minRow, maxRow + 1):
        for col in range(minCol, maxCol + 1):
            charIndex = pixelsToDecimal(
                getSurroundingPixels(col, row, imgDict, defaultChar))
            changes[(col, row)] = algoStr[charIndex]
    return changes


def countLitPixels(imgDict):
    return len(list(filter(lambda pixel: pixel == '#', imgDict.values())))


def allBorders(imgDict, match):
    minCol, maxCol, minRow, maxRow = getMinMaxCoords(imgDict)
    #top border (minCol,minRow to maxCol,minRow)
    for col in range(minCol, maxCol + 1):
        if imgDict[(col, minRow)] != match:
            return False
    #bottom border
    for col in range(minCol, maxCol + 1):
        if imgDict[(col, maxRow)] != match:
            return False
    #left border
    for row in range(minRow, maxRow + 1):
        if imgDict[(minCol, row)] != match:
            return False
    #right border
    for row in range(minRow, maxRow + 1):
        if imgDict[(maxCol, row)] != match:
            return False
    return True


def allBordersDark(imgDict):
    return allBorders(imgDict, '.')


def allBordersLit(imgDict):
    return allBorders(imgDict, '#')


def processImage(imgDict, algo, steps):
    for step in range(steps):
        defaultChar = '.'
        # if the first char in the algorithm is '#' sequences of '.........' will turn everything into #, so we invert it
        if algo[0] == '#' and step % 2 == 1:
            defaultChar = '#'
        imgDict = enhanceImage(imgDict, algo, defaultChar)
    return countLitPixels(imgDict)


#data = aoc.readFileToStringList("20.test.txt")
data = aoc.readFileToStringList("20.txt")
imgAlgo = data[0]
imageData = data[2:]
imageDict = loadImageDict(imageData)

# part 1
ANS1 = processImage(imageDict, imgAlgo, 2)

#part 2
imageDict = loadImageDict(imageData)
ANS2 = processImage(imageDict, imgAlgo, 50)

print("ans1={}".format(ANS1))  # 5361
print("ans2={}".format(ANS2))  # 16826
