import aoc
# Giant Squid (Bingo)
ANS1 = 0
ANS2 = 0

#input = aoc.readFileToStringList("4.test.txt")
input = aoc.readFileToStringList("4.1.txt")

# first line, convert to str list, then int list
bingoNumbers = list(map(int, input[0].split(',')))

# rest of document:
# - filter out empty lines
# - then split each line by spaces and convert to int lists
# You end up with a list that should have a multiple of 5 given each board has 5 rows.
bingoBoardsData = list(
    map(lambda line: list(map(int, line.split())),
        filter(lambda line: len(line) > 0, input[2:])))

class BingoBoard:
    def __init__(self) -> None:
        self.numRows = []
        self.reset()

    def bingoed(self):
        return self.__bingoed

    def reset(self) -> None:
        self.initMarkedRows()
        self.__bingoed = False

    def initMarkedRows(self) -> None:
        self.markedRows = [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]]

    def addRow(self, intList):
        self.numRows.append(intList)

    def tryToMarkNumber(self, number):
        for row in range(5):
            for column in range(5):
                if self.numRows[row][column] == number:
                    self.markedRows[row][column] = 1
                    return True
        return False

    def checkForBingo(self):
        # easy check if any of the markedRows adds up to 5
        for row in self.markedRows:
            if sum(row) == 5:
                self.__bingoed = True
                return True

        # now check columns
        for column in range(5):
            total = 0
            for row in range(5):
                total += self.markedRows[row][column]
            if total == 5:
                self.__bingoed = True
                return True
        return False

    def sumUnmarkedNumbers(self):
        total = 0
        for row in range(5):
            for column in range(5):
                if self.markedRows[row][column] == 0:
                    total += self.numRows[row][column]
        return total

# Load Bingo Boards
i = 0
bingoBoards = []
bufferBoard = BingoBoard()
for row in bingoBoardsData:
    bufferBoard.addRow(row)
    if i == 4:
        bingoBoards.append(bufferBoard)
        bufferBoard = BingoBoard()
        i = 0
    else:
        i += 1


def playBingo(bingoNumbers, bingoBoards, returnLastBoardToWin=False):
    lastWinningNumber = -1
    lastWinningBoard = None
    for number in bingoNumbers:
        for board in bingoBoards:
            if not board.bingoed() and board.tryToMarkNumber(
                    number) and board.checkForBingo():
                if not returnLastBoardToWin:
                    return (number, board)
                else:
                    lastWinningNumber = number
                    lastWinningBoard = board
    return (lastWinningNumber, lastWinningBoard)


# Part 1
number, winningBoard = playBingo(bingoNumbers, bingoBoards)
ANS1 = winningBoard.sumUnmarkedNumbers() * number
print("ans1={}".format(ANS1)) # ans1=22680

# Part 2
for board in bingoBoards:
    board.reset()
number, winningBoard = playBingo(bingoNumbers, bingoBoards, True)
ANS2 = winningBoard.sumUnmarkedNumbers() * number
print("ans2={}".format(ANS2)) # ans2=16168