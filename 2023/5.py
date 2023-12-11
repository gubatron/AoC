from aoc import read_file_to_string_list
from dataclasses import dataclass
from concurrent.futures import ProcessPoolExecutor, ThreadPoolExecutor


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
        return seed


def parse_seeds(lines):
    return list(map(int, lines[0].split(": ")[1].split(' ')))


def merge_ranges(ranges):
    # Sort the list by the start of each range
    sorted_ranges = sorted(ranges, key=lambda x: x[0])

    # Initialize the merged list with the first range
    merged = [sorted_ranges[0]]

    for current_start, current_length in sorted_ranges[1:]:
        # Calculate the end of the last range in merged list
        last_end = merged[-1][0] + merged[-1][1] - 1

        # Calculate the end of the current range
        current_end = current_start + current_length - 1

        # Check if there is an overlap
        if current_start <= last_end:
            # Merge the overlapping ranges
            new_end = max(last_end, current_end)
            merged[-1] = (merged[-1][0], new_end - merged[-1][0] + 1)
        else:
            # If no overlap, just add the current range to the merged list
            merged.append((current_start, current_length))

    return merged


def parse_seeds_2(lines):
    seeds = []
    seed_numbers = parse_seeds(lines)
    # get every 2 seeds until the end
    seed_ranges = list(zip(seed_numbers[::2], seed_numbers[::-2][::-1]))
    seed_ranges = seed_ranges[:-1]
    merged_ranges = merge_ranges(seed_ranges)
    return merged_ranges


def parse_seed_maps(lines):
    seed_maps = []
    i = 2
    while i < len(lines):
        line = lines[i]
        current_seed_map = SeedMap("", [])
        if '-to-' in line:
            current_seed_map.name = line
            i += 1
            while i < len(lines) and lines[i] != '':
                line = lines[i]
                seed_range_data = list(map(int, line.split(' ')))
                current_seed_range = SeedRange(seed_range_data[0], seed_range_data[1], seed_range_data[2])
                current_seed_map.ranges.append(current_seed_range)
                i += 1
            seed_maps.append(current_seed_map)
        i += 1

    return seed_maps


def feed_seed_through_seed_map_pipeline(seed, seed_maps):
    for sm in seed_maps:
        seed = (sm.get_dest(seed))
    return seed


def find_lowest_location_in_seed_list(seeds, seed_maps):
    # we feed each seed through all the seed maps and we keep the lowest result
    lowest_location = 999999999999999999
    for seed in seeds:
        lowest_location = min(lowest_location, feed_seed_through_seed_map_pipeline(seed, seed_maps))
    return lowest_location


def find_lowest_location_in_seed_range(seed_range, seed_maps):
    # we feed each seed through all the seed maps and we keep the lowest result
    lowest_location = 999999999999999999
    # a range is a tuple of (start, length)
    for seed in range(seed_range[0], seed_range[0] + seed_range[1]):
        lowest_location = min(lowest_location, feed_seed_through_seed_map_pipeline(seed, seed_maps))
    return lowest_location


def find_lowest_location_2(merged_ranges, seed_maps):
    # Function to be executed in each thread, accepting a dictionary
    def process_seed_range(params):
        return find_lowest_location_in_seed_range(params['range'], params['seed_maps'])

    # Create a list of dictionaries, each containing a seed along with the seed_maps and cache
    params_list = [{'range': range, 'seed_maps': seed_maps} for range in merged_ranges]

    # Using ThreadPoolExecutor to parallelize
    with ThreadPoolExecutor(max_workers=8) as executor:
        # Map each params dictionary to the process_seed function
        results = executor.map(lambda params: process_seed_range(params), params_list)

    # Find the lowest result from all threads
    lowest_location = min(results, default=999999999999999999)
    return lowest_location


def part1(data_file):
    lines = read_file_to_string_list(data_file)
    seeds = parse_seeds(lines)
    seed_maps = parse_seed_maps(lines)
    print(f"Part 1: Lowest location = {find_lowest_location_in_seed_list(seeds, seed_maps)}")


def part2(data_file):
    lines = read_file_to_string_list(data_file)
    merged_ranges = parse_seeds_2(lines)
    print(f"merged_ranges = {merged_ranges}")
    seed_maps = parse_seed_maps(lines)
    print(f"Part 2: Lowest location = {find_lowest_location_2(merged_ranges, seed_maps)}")


data_file = "5.txt"
part1(data_file)
part2(data_file)
