from math import ceil

from aoc import read_file_to_string_list


def calc_possible_num_wins(total_time, total_distance, debug=False):
    wins = 0
    speed = ceil(total_distance / total_time)
    while speed < total_time:
        test_distance = (total_time - speed) * speed
        if test_distance > total_distance:
            wins += 1
            if debug:
                print(
                    f"would win with speed = {speed} @ dist: {test_distance} (total_time {total_time}, total_distance {total_distance})")
        speed += 1
    return wins


def part1(data):
    times = [int(s) for s in data[0].split(':')[1].strip().split(' ') if s]
    distances = [int(s) for s in data[1].split(':')[1].strip().split(' ') if s]
    races = list(zip(times, distances))
    ans = 1
    for total_time, total_distance in races:
        wins = calc_possible_num_wins(total_time, total_distance, False)
        ans *= wins
        print(f"total_time = {total_time}, total_distance = {total_distance}")
        print(f"possible_num_wins = {wins}")
        print("")

    print(f"Part 1: {ans}")


def part2(total_time, total_distance):
    print(f"Part 2: {calc_possible_num_wins(total_time, total_distance, False)}")


data = read_file_to_string_list('6.txt')
part1(data)
part2(49877895, 356137815021882)
