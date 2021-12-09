import aoc
from itertools import permutations

data = aoc.readFileToStringList("8.txt")


def parser(line):
    signals, output = line.split(' | ')
    signals = signals.split(' ')
    output = output.split(' ')
    return signals, output


# [ [string, string,...], [string, string,]]
outputs_only = (list(map(lambda a: a[1], map(parser, data))))

# number of signals that map to an output number(s)
sigs2num = {2: [1], 3: [7], 4: [4], 7: [8], 5: [2, 3, 5], 6: [0, 6, 9]}


def countDigits1478(output):
    global sigs2num
    return len(
        list(filter(lambda digit: len(sigs2num[len(digit)]) == 1, output)))


# All of the digits within a display use the same connections
ANS1 = sum(map(countDigits1478, outputs_only))  #525

signals_outputs = list(map(parser, data))

# how things would be lit up
number_to_segments = {
    # unique ones
    1: [3, 6],  # 2 segments
    7: [1, 3, 6],  # 3 segments
    4: [2, 3, 4, 6],  # 4 segments
    8: [1, 2, 3, 4, 5, 6, 7],  # 7 segments
    # ambiguous ones
    2: [1, 3, 4, 5, 7],  # 5 segments
    3: [1, 3, 4, 6, 7],  # 5 segments
    5: [1, 2, 4, 6, 7],  # 5 segments
    0: [1, 2, 3, 5, 6, 7],  # 6 segments
    6: [1, 2, 4, 5, 6, 7],  # 6 segments
    9: [1, 2, 3, 4, 6, 7],  # 6 segments
}

# reverse map, segments lit to number
segments_to_number = {str(v): k for k, v in number_to_segments.items()}


def decode(signal, decoding_map) :#-> list[int]:
    global sigs2num, number_to_segments, segments_to_number
    decoded = []
    for letters in signal:
        letters = ''.join(sorted(letters))
        if len(sigs2num[len(letters)]) == 1:
            found_number = sigs2num[len(letters)][0]
            decoded.append(found_number)
        else:
            segments = []
            for char in letters:
                if char in decoding_map:
                    segments.append(decoding_map[char])
                str_segments = str(sorted(segments))
            if str_segments in segments_to_number:
                output_number = segments_to_number[str_segments]
                decoded.append(output_number)
    return decoded


def findDecodedOutputNumber(signal_patterns: list[str],
                            output_values: list[str]) -> int:
    signal_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
    signal_permutations_iterator = permutations([1, 2, 3, 4, 5, 6,
                                                 7])  # Only 7 factorial = 5040
    found_decoding_map = False
    char_to_segment_number = {}
    while not found_decoding_map:
        char_to_segment_number = dict(
            zip(signal_letters, next(signal_permutations_iterator)))
        decoded_signals = decode(signal_patterns, char_to_segment_number)
        decoded_outputs = decode(output_values, char_to_segment_number)
        found_decoding_map = len(signal_patterns) == len(
            decoded_signals) and len(output_values) == len(decoded_outputs)
    return int(''.join(map(str, decoded_outputs)))


ANS2 = sum(
    map(lambda item: findDecodedOutputNumber(item[0], item[1]),
        signals_outputs))

print("ans1={}".format(ANS1))  # 525
print("ans2={}".format(ANS2))  # 1083859