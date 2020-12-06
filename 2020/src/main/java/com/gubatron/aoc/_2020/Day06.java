package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringList;
import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day06 {
    static class Group {
        List<String> answers = new ArrayList<>();

        Group(List<String> answers_p) {
            answers.addAll(answers_p);
        }

        long countCommonAnswers() {
            if (answers == null || answers.size() == 0) {
                return 0;
            }
            if (answers.size() == 1) {
                return answers.get(0).length();
            }
            Map<Integer, Integer> commonChars = new HashMap<>();
            answers.forEach(s -> {
                s.chars().boxed().forEach(c ->
                        commonChars.merge(c, 1, Integer::sum));
            });
            return commonChars.entrySet().stream().filter(entry -> entry.getValue() == answers.size()).count();
        }
    }

    public static long part1(List<String> groupLines) {
        return groupLines.stream().map(groupLine -> groupLine.chars().filter(c -> c != '\n').distinct().count()).reduce(Long::sum).get();
    }

    // Convert List<String> -> Stream<Group> -> Stream<Long> -> sum it
    public static long part2(List<String> groupLines) {
        return groupLines.stream().map(groupLine -> {
                    List<String> answers = new ArrayList<>();
                    Collections.addAll(answers, groupLine.split("\n"));
                    return new Group(answers);
                }
        ).map(Group::countCommonAnswers).reduce(Long::sum).get();
    }

    public static void main(String[] args) throws IOException {
        List<String> splitLines = readStringsBySeparator(new File("resources/input_day_06.txt"), "\n\n");
        System.out.println("DAY 06");
        System.out.println("Part 1: " + part1(splitLines) + " (Expected: 6297)");
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(splitLines) + " (Expected: 3158)");
    }
}