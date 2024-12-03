import sys
from pathlib import Path
# Add the ../ directory to the Python path
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc as aoc
# Giant Squid (Bingo)
input = aoc.read_file_to_string_list("4.1.txt")
bingoNumbers = list(map(int, input[0].split(',')))

# filter out empty lines, then split each line by spaces and convert to int lists
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
            if sum(row[column] for row in self.markedRows) == 5:
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
    i += 1
    if i == 5:
        bingoBoards.append(bufferBoard)
        bufferBoard = BingoBoard()
        i = 0


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
print("ans1={}".format(ANS1))  # ans1=22680

# Part 2
for board in bingoBoards:
    board.reset()
number, winningBoard = playBingo(bingoNumbers, bingoBoards, True)
ANS2 = winningBoard.sumUnmarkedNumbers() * number
print("ans2={}".format(ANS2))  # ans2=16168
