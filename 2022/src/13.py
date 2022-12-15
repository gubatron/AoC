# Day 13: Distress Signal
import json


def split_file_by_empty_lines(file_name):
    with open(file_name, 'r') as f:
        lines = f.read()
        return lines.split('\n\n')


def compare(left, right, indent):
    indentation = ' ' * indent
    print(indentation + '- Compare ' + str(left) + ' vs ' + str(right))

    if type(left) is int and type(right) is int:
        return left - right

    if type(left) is int and type(right) is list:
        return compare([left], right, indent + 2)

    if type(left) is list and type(right) is int:
        return compare(left, [right], indent + 2)

    for left_item, right_item in zip(left, right):
        cmp = compare(left_item, right_item, indent + 2)
        if cmp != 0:
            return cmp
    return len(left) - len(right)


if __name__ == '__main__':
    signals = list(
        map(lambda x: [json.loads(signal) for signal in x.split('\n')],
            split_file_by_empty_lines('../inputs/13.txt')
            )
    )

    results = 0
    for i, (left, right) in enumerate(signals):
        print(f"== Pair {i + 1} == ")
        if compare(left, right, 0) < 0:
            print(f"Pair {i + 1} is correct!")
            results += i + 1
        print()

    # test:13
    # result: 5366
    print(f"Part 1: {results}")
