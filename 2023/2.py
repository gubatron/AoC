from aoc import read_file_to_string_list
import re


def parse_game_results(text):
    # Extracting game ID
    game_id_pattern = r"Game (\d+)"
    game_id_match = re.search(game_id_pattern, text)
    game_id = game_id_match.group(1) if game_id_match else None

    # Splitting the text into laps
    laps = text.split(';')

    # Pattern to match color and number pairs
    color_pattern = r"(\d+) (red|green|blue)"

    lap_results = []

    for lap in laps:
        # Finding all color and number pairs in the lap
        colors = re.findall(color_pattern, lap)

        # Summing the values for each color
        lap_dict = {'red': 0, 'green': 0, 'blue': 0}
        for number, color in colors:
            if color in lap_dict:
                lap_dict[color] += int(number)
            else:
                lap_dict[color] = int(number)

        lap_results.append(lap_dict)

    return int(game_id), lap_results


def valid_lap(lap):
    result = lap['red'] <= 12 and lap['green'] <= 13 and lap['blue'] <= 14
    return result


def valid_game(laps):
    for lap in laps:
        if not valid_lap(lap):
            return False
    return True


def part1():
    games = read_file_to_string_list("2.txt")
    sum_valid_game_ids = 0

    for game_line in games:
        game_id, laps = parse_game_results(game_line)
        if valid_game(laps):
            sum_valid_game_ids += game_id
    print(sum_valid_game_ids)


def max_colors(laps):
    best = {'red': 0, 'green': 0, 'blue': 0}
    for lap in laps:
        best['red'] = max(lap['red'], best['red'])
        best['green'] = max(lap['green'], best['green'])
        best['blue'] = max(lap['blue'], best['blue'])
    return best


def power(laps):
    p = 1
    for k, v in laps.items():
        p = p * v
    return p


def part2():
    games = read_file_to_string_list("2.txt")
    power_sum = 0
    for game_line in games:
        game_id, laps = parse_game_results(game_line)
        power_sum += power(max_colors(laps))
    print(power_sum)


part1()
part2()
