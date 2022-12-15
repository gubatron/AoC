# Day 13: Distress Signal
import json


def split_file_by_empty_lines(file_name):
    with open(file_name, 'r') as f:
        lines = f.read()
        return lines.split('\n\n')


def compare(left, right):
    if type(left) is int and type(right) is int:
        return left - right

    if type(left) is int and type(right) is list:
        return compare([left], right)

    if type(left) is list and type(right) is int:
        return compare(left, [right])

    for left_item, right_item in zip(left, right):
        cmp = compare(left_item, right_item)
        if cmp != 0:
            return cmp
    return len(left) - len(right)


if __name__ == '__main__':
    signals = list(
        map(lambda x: [json.loads(signal) for signal in x.split('\n')],
            split_file_by_empty_lines('../inputs/13.txt')
            )
    )

    part_1 = 0
    packets = [[[2]],[[6]]]

    for i, (left, right) in enumerate(signals):
        packets.append(left)
        packets.append(right)

        print(f"== Pair {i + 1} == ")
        if compare(left, right) < 0:
            print(f"Pair {i + 1} is correct!")
            part_1 += i + 1
        print()

    # test:13
    # result: 5366
    print(f"Part 1: {part_1}")

    import functools
    packets.sort(key=functools.cmp_to_key(compare))
    part_2 = 1
    for i, p in enumerate(packets):
        if p == [[2]] or p == [[6]]:
            part_2 *= (i+1)
        print(str(i+1) + ' ' + str(p))

    # test: 140
    # result: 23391
    print(f"Part 2: {part_2}")