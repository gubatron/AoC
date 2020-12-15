package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day15 {

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_15.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_15.txt"), "\n");
        System.out.println("DAY 15 - Rambunctious Recitation");
        List<Integer> numbers = Arrays.stream(lines.get(0).split(",")).map(Integer::parseInt).collect(Collectors.toList());
        System.out.println("Part 1: " + part1(numbers)); // 662
        System.out.println("==============================");
        numbers = Arrays.stream(lines.get(0).split(",")).map(Integer::parseInt).collect(Collectors.toList());
        System.out.println("Part 2: " + part2(numbers)); // 37312
    }

    public static long part1(List<Integer> spokenNumbers) {
        return elfMemoryGame(spokenNumbers, 2020);
    }

    public static long part2(List<Integer> spokenNumbers) {
        return elfMemoryGame(spokenNumbers, 30000000);
    }

    private static void speakNumber(int turn, int n, HashMap<Integer, List<Integer>> numberTurns, List<Integer> spokenNumbers) {
        if (!numberTurns.containsKey(n)) {
            List<Integer> turns = new ArrayList<>();
            turns.add(turn);
            numberTurns.put(n, turns);
        } else {
            numberTurns.get(n).add(turn);
        }
        if (spokenNumbers != null) {
            spokenNumbers.add(n);
        }
    }

    private static long elfMemoryGame(List<Integer> spokenNumbers, int targetTurn) {
        HashMap<Integer, List<Integer>> numberTurns = new HashMap<>();
        int turn = 1;
        int last_number = 0;
        for (int n : spokenNumbers) {
            speakNumber(turn, n, numberTurns, null);
            last_number = n;
            turn++;
        }

        while (turn <= targetTurn) {
            List<Integer> turns = numberTurns.get(last_number);
            if (turns.size() == 1) {
                speakNumber(turn, 0, numberTurns, spokenNumbers);
                last_number = 0;
            } else {
                int previous = turns.get(turns.size() - 1);
                int before_previous = turns.get(turns.size() - 2);
                int next_number = previous - before_previous;
                speakNumber(turn, next_number, numberTurns, spokenNumbers);
                last_number = next_number;
            }
            turn++;
        }
        return last_number;
    }
}
