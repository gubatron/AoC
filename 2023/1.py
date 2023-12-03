from aoc import read_file_to_string_list


def get_digits(input):
    first, second = None, None
    max_offset = len(input) - 1
    i = 0
    j = max_offset

    while i < max_offset and j > 0:
        left = input[i]
        right = input[j]
        if first is None and left.isdigit():
            first = left
        if second is None and right.isdigit():
            second = right
        i += 1
        j -= 1

    if second is None:
        second = first
    if first is None:
        first = second
    return first + second


def part1():
    input = read_file_to_string_list("1.txt")
    print(sum(list(map(int, list(map(get_digits, input))))))


def extract_digits(input):
    digits = []
    for i, c in enumerate(input):
        if c.isdigit():
            digits.append(c)
        else:
            for j, spelled in enumerate(
                    ['zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']):
                if input[i:].startswith(spelled):
                    digits.append(str(j))
    result = ''.join(digits)
    if len(result) == 1:
        result = result + result
    result = result[0] + result[-1]
    return int(result)


def part2():
    input = read_file_to_string_list("1.txt")
    print(sum(map(extract_digits, input)))


part1()
part2()
