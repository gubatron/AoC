from aoc import read_file_to_string_list
from dataclasses import dataclass


@dataclass(frozen=True)
class Card:
    winning_numbers: list
    have_numbers: list

    def points(self):
        # count how many of the have_numbers are in winning_numbers
        n = len(set(self.winning_numbers) & set(self.have_numbers))
        if n == 0:
            return 0
        result = 2 ** (n - 1)
        return result


def line_to_card(line):
    parts = line.split(': ')
    numbers = parts[1].split(' | ')
    winners = list(map(int, numbers[0].strip().split()))
    have = list(map(int, numbers[1].strip().split()))
    return Card(winners, have)


def part1():
    card_lines = read_file_to_string_list('4.txt')
    pile = list(map(line_to_card, card_lines))
    print('Part 1: ' + str(sum(map(lambda card: card.points(), pile))))


part1()
