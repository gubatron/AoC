package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Hashtable;
import java.util.List;
import java.util.Set;

import static com.gubatron.aoc._2020.Utils.readStringList;

public class Day06 {
    static class Group {
        List<String> answers;

        int countCommonAnswers() {
            if (answers.size() == 1) {
                return answers.get(0).length();
            }

            Hashtable<String, Integer> commonChars = new Hashtable<>();
            for (String answer : answers) {
                char[] chars = answer.toCharArray();
                for (char c : chars) {
                    String key = String.valueOf(c);
                    if (commonChars.containsKey(key)) {
                        commonChars.put(key, 1 + commonChars.get(key));
                    } else {
                        commonChars.put(key, 1);
                    }
                }
            }
            Set<String> chars = commonChars.keySet();
            int total_common_ans = 0;
            for (String c : chars) {
                // number of common answers has to match group size
                // ALL have to say yes
                if (commonChars.get(c) == answers.size()) {
                    total_common_ans++;
                }
            }
            return total_common_ans;
        }
    }

    public static long part1(List<String> lines) {
        List<String> groups = new ArrayList<>(0);
        StringBuilder collector = new StringBuilder();
        for (String line : lines) {
            if (line.isEmpty()) {
                groups.add(collector.toString());
                collector = new StringBuilder();
            } else {
                collector.append(line);
            }
        }
        if (!collector.isEmpty()) {
            groups.add(collector.toString());
        }

        int total = 0;
        for (String g : groups) {
            total += countDistinctAnswers(g);
        }
        return total;
    }

    public static long part2(List<String> lines) {
        List<Group> groups = new ArrayList<>();
        List<String> lastGroupAnswers = new ArrayList<>();
        for (String line : lines) {
            if (!line.isEmpty()) {
                lastGroupAnswers.add(line);
            } else {
                Group group = new Group();
                group.answers = new ArrayList<>(lastGroupAnswers);
                groups.add(group);
                lastGroupAnswers.clear();

            }
        }
        if (!lastGroupAnswers.isEmpty()) {
            Group group = new Group();
            group.answers = new ArrayList<>(lastGroupAnswers);
            groups.add(group);
        }

        int totalAnswers = 0;
        int i=0;
        for (Group g : groups) {
            int commonAnswers = g.countCommonAnswers();
            totalAnswers += commonAnswers;
        }

        return totalAnswers;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringList(new File("resources/sample_day_06.txt"));
        List<String> lines = readStringList(new File("resources/input_day_06.txt"));
        System.out.println("DAY 06");
        System.out.println("Part 1: " + part1(lines));
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(lines));
    }

    private static long countDistinctAnswers(String group) {
        return group.chars().distinct().count();
    }
}
