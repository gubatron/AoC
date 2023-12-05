from dataclasses import dataclass
from aoc import read_char_matrix, print_matrix, get_surrounding_coords


@dataclass(frozen=True)
class Part:
    row: int
    col: int
    number: int


def get_matrix_num_cols(m):
    return len(m[0])


def get_part_from_coord(row, col, char_matrix):
    # we read all the digits left and right around the given digit at row_col
    digit_center = char_matrix[row][col]
    digits_left = []
    digits_right = []
    result_digits = []
    # read left
    start_col = col
    start_row = row
    c = col - 1
    while in_bounds(row, c, char_matrix) and char_matrix[row][c].isdigit():
        # we insert the digits at digits_left[0] because we want the digits to be in order
        digits_left.insert(0, char_matrix[row][c])
        start_col = c
        c = c - 1
    # read right
    c = col + 1
    while in_bounds(row, c, char_matrix) and char_matrix[row][c].isdigit():
        digits_right.append(char_matrix[row][c])
        c = c + 1
    if len(digits_left) > 0:
        result_digits += digits_left
    result_digits.append(digit_center)
    if len(digits_right) > 0:
        result_digits += digits_right
    return Part(start_row, start_col, int(''.join(result_digits)))


def in_bounds(sr, sc, char_matrix):
    num_rows = len(char_matrix)
    num_cols = get_matrix_num_cols(char_matrix)
    return 0 <= sr < num_rows and sc >= 0 and sc < num_cols


def day3():
    char_matrix = read_char_matrix('3.txt')
    print_matrix(char_matrix)

    # iterate row by row and find digits around characters other than digits and '.'
    # for each character, find the surrounding coordinates
    # for each surrounding coordinate, check if it's a digit
    num_rows = len(char_matrix)
    num_cols = get_matrix_num_cols(char_matrix)
    part_numbers_sum = 0
    gear_ratios = 0
    for r in range(num_rows):
        for c in range(num_cols):
            if char_matrix[r][c] == '.' or char_matrix[r][c].isdigit():
                continue

            is_gear_candidate = char_matrix[r][c] == '*'
            surrounding_coords = get_surrounding_coords(r, c, char_matrix, includeDiagonals=True)
            surrounding_parts = set()

            for (sr, sc) in surrounding_coords:
                if in_bounds(sr, sc, char_matrix) and char_matrix[sr][sc].isdigit():
                    part = get_part_from_coord(sr, sc, char_matrix)
                    surrounding_parts.add(part)

            if len(surrounding_parts) > 0:
                print(f'({char_matrix[r][c]}@{r}, {c}): {surrounding_parts}')

                for part in surrounding_parts:
                    part_numbers_sum += part.number
            if is_gear_candidate and len(surrounding_parts) == 2:
                it = iter(surrounding_parts)
                first_part_number = next(it).number
                second_part_number = next(it).number
                gear_ratios += first_part_number * second_part_number

    print(f'part1: {part_numbers_sum}')
    print(f'part2: {gear_ratios}')


day3()
