from aoc import read_file_to_string_list
from dataclasses import dataclass


@dataclass
class SeedMap:
    name: str
    ranges: list

    def get_dest(self, seed):
        for r in self.ranges:
            if r.in_source_range(seed):
                return r.get_dest(seed)
        return seed


@dataclass(frozen=True)
class SeedRange:
    dest: int
    source: int
    length: int

    def in_source_range(self, seed):
        # considering the length of the range is it in the source range?
        return seed >= self.source and seed <= self.source + self.length

    def get_dest(self, seed):
        if self.in_source_range(seed):
            return seed + (self.dest - self.source)
        else:
            return seed


def parse_seeds(lines):
    return list(map(int, lines[0].split(": ")[1].split(' ')))


def parse_seed_maps(lines):
    seed_maps = []
    i = 2
    while i < len(lines):
        line = lines[i]
        current_seed_map = SeedMap("", [])
        if '-to-' in line:
            current_seed_map.name = line
            i += 1
            print(i)
            while i < len(lines) and lines[i] != '':
                line = lines[i]
                seed_range_data = list(map(int, line.split(' ')))
                current_seed_range = SeedRange(seed_range_data[0], seed_range_data[1], seed_range_data[2])
                print(current_seed_range)
                current_seed_map.ranges.append(current_seed_range)
                i += 1
            seed_maps.append(current_seed_map)
        i += 1
    return seed_maps


def feed_seed_through_seed_map_pipeline(seed, seed_maps):
    for sm in seed_maps:
        seed = sm.get_dest(seed)
    return seed


def find_lowest_location(seeds, seed_maps):
    # we feed each seed through all the seed maps and we keep the lowest result
    return min(map(lambda seed: feed_seed_through_seed_map_pipeline(seed, seed_maps), seeds))


def part1():
    lines = read_file_to_string_list("5.txt")
    seeds = parse_seeds(lines)
    seed_maps = parse_seed_maps(lines)
    for sm in seed_maps:
        print(sm.name)
        for r in sm.ranges:
            print(r)
        print()

    for seed in seeds:
        print(f"Seed: {seed} -> {feed_seed_through_seed_map_pipeline(seed, seed_maps)}")

    print(f"Lowest location: {find_lowest_location(seeds, seed_maps)}")


part1()
