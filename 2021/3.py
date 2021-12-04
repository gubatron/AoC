import aoc

ANS1 = 0
ANS2 = 0

report = aoc.readFileToStringList("3.1.txt")


def mostCommonBitAt(report, i):
    n = len(report)
    ones = 0
    zeroes = 0
    for x in report:
        if x[i] == '1':
            ones += 1
        else:
            zeroes += 1
    if zeroes > ones:
        return '0'
    return '1'


def mostCommonBits(report):
    commonBits = ""
    wordLength = len(report[0])
    for i in range(wordLength):
        commonBits += mostCommonBitAt(report, i)
    return commonBits


def invertBinaryString(s):
    return ''.join('1' if x == '0' else '0' for x in s)


# PART 1
gammaBinary = mostCommonBits(report)
epsilonBinary = invertBinaryString(gammaBinary)
gammaDec = int(gammaBinary, 2)
epsilonDec = int(epsilonBinary, 2)
powerConsumption = gammaDec * epsilonDec
ANS1 = powerConsumption

# PART 2
# life support rating = oxygen generator rating by CO2 scrubber rating
# oxygen support comes out most common bit words
# CO2 out of least common bit words
# filter words with most common bit at: [words],i -> [words]


def oxygenGeneratorRating(report, i):
    if len(report) == 1:
        return report[0]
    commonBit = mostCommonBitAt(report, i)
    report = list(filter(lambda word: word[i] == commonBit, report))
    return oxygenGeneratorRating(report, i + 1)


def leastCommonBitAt(report, i):
    if mostCommonBitAt(report, i) == '1':
        return '0'
    return '1'


def CO2ScrubberRating(report, i):
    if len(report) == 1:
        return report[0]
    uncommonBit = leastCommonBitAt(report, i)
    report = list(filter(lambda word: word[i] == uncommonBit, report))
    return CO2ScrubberRating(report, i + 1)


O2RatingBinary = oxygenGeneratorRating(report, 0)
CO2ScrubberRatingBinary = CO2ScrubberRating(report, 0)
O2RatingDec = int(O2RatingBinary, 2)
CO2ScrubberRatingDec = int(CO2ScrubberRatingBinary, 2)
print("oxygenGeneratorRating={} -> {}".format(O2RatingBinary, O2RatingDec))
print("CO2ScrubberRating={} -> {}".format(CO2ScrubberRatingBinary,
                                          CO2ScrubberRatingDec))
ANS2 = O2RatingDec * CO2ScrubberRatingDec

print("ANS1={}".format(ANS1))  #ANS1=3923414
print("ANS2={}".format(ANS2))  #ANS2=5852595