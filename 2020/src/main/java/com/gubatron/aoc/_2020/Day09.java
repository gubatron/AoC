package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day09 {
    public static long part1(int preamble, List<Long> numbers) {
        for (int XMAS_POINTER = preamble + 1; XMAS_POINTER < numbers.size(); XMAS_POINTER++) {
            List<Long> allSums = new ArrayList<>();
            long candidate = numbers.get(XMAS_POINTER);
            for (int i = XMAS_POINTER - preamble; i < XMAS_POINTER; i++) {
                for (int j = i + 1; j < XMAS_POINTER; j++) {
                    long sum = numbers.get(i) + numbers.get(j);
                    allSums.add(sum);
                }
            }
            if (!allSums.contains(candidate)) {
                return candidate;
            }
        }
        return -1;
    }

    public static long part2(int preamble, List<Long> numbers) {
        long invalid_checksum = part1(preamble, numbers);
        long invalid_checksum_pos = numbers.indexOf(invalid_checksum);
        // find contiguous set of numbers that add up to invalid checksum
        int windowSize = 2;
        while (true) {
            for (int i = 0; i < invalid_checksum_pos; i++) {
                long sum = sumSubset(i, i + windowSize, numbers);
                if (sum == invalid_checksum) {
                    long min = minInSubset(i, i + windowSize, numbers);
                    long max = maxInSubset(i, i + windowSize, numbers);
                    return min + max;
                }
            }
            windowSize++;
        }
    }

    private static long minInSubset(int start, int endExclusive, List<Long> numbers) {
        long min = Long.MAX_VALUE;
        for (int i = start; i < endExclusive; i++) {
            if (numbers.get(i) < min) {
                min = numbers.get(i);
            }
        }
        return min;
    }

    private static long maxInSubset(int start, int endExclusive, List<Long> numbers) {
        long max = 0;
        for (int i = start; i < endExclusive; i++) {
            if (numbers.get(i) > max) {
                max = numbers.get(i);
            }
        }
        return max;
    }

    public static long sumSubset(int start, int endExclusive, List<Long> numbers) {
        long sum = 0;
        for (int i = start; i < endExclusive; i++) {
            sum += numbers.get(i);
        }
        return sum;
    }

    public static void main(String[] args) throws IOException {
        // SAMPLE
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_09.txt"),"\n");
        //int preamble = 5;

        // INPUT
        List<String> lines = readStringsBySeparator(new File("resources/input_day_09.txt"), "\n");
        int preamble = 25;

        List<Long> numbers = lines.stream().map(l -> Long.parseLong(l)).collect(Collectors.toList());

        System.out.println("DAY 09 -  Encoding Error");
        System.out.println("Part 1: " + part1(preamble, numbers)); // 26134589
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(preamble, numbers)); // 3535124
    }
}
