from aoc import read_file_to_string_list
from dataclasses import dataclass


@dataclass
class Card:
    winning_numbers: list
    have_numbers: list
    card_number: int = 0
    copies: int = 1

    def points(self):
        # count how many of the have_numbers are in winning_numbers
        n = self.num_matching_cards()
        if n == 0:
            return 0
        result = 2 ** (n - 1)
        return result

    def num_matching_cards(self):
        return len(set(self.winning_numbers) & set(self.have_numbers))

    def __repr__(self):
        og = 'original'
        if not self.is_original:
            og = 'copy'

        return f'<{self.card_number}: {self.num_matching_cards()} matches - ({og})>'


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


def part2():
    card_lines = read_file_to_string_list('4.txt')
    pile = list(map(line_to_card, card_lines))
    card_dict = {}
    for i, card in enumerate(pile):
        card.card_number = i + 1
        card_dict[card.card_number] = card

    for card in pile:
        matches = card.num_matching_cards()
        if matches > 0:
            i = card.card_number + 1
            for card_number in range(i, i + matches):
                if card_number in card_dict:
                    card_dict[card_number].copies += card.copies
    # sum all the card copies
    print('Part 2: ' + str(sum(map(lambda card: card.copies, pile))))


part1()
part2()
