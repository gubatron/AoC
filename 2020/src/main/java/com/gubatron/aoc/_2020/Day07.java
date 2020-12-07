package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.stream.Collectors;

public class Day07 {
    static class BagRule {
        String color;
        // color -> how many bags contained
        HashMap<String, Integer> containedBags = new HashMap<>();

        BagRule(String line) {
            String cleanLine = line.replace(" bags contain ", ":").
                    replace("bags, ", "").
                    replace("bag, ", "").
                    replace(" bags.", "").
                    replace("bag.", "");

            String[] split = cleanLine.split(":");
            color = split[0];

            String rest = split[1].strip();
            if (rest.contains("no other")) {
                return;
            }

            String[] singles = rest.split(" ");
            for (int i = 0; i < singles.length; i += 3) {
                int amount = Integer.parseInt(singles[i]);
                String color = singles[i + 1] + " " + singles[i + 2];
                containedBags.put(color, amount);
            }
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof BagRule)) return false;
            BagRule bagRule = (BagRule) o;
            return Objects.equals(color, bagRule.color);
        }

        @Override
        public int hashCode() {
            return Objects.hash(color);
        }
    }

    public static void findMatchingRule(List<BagRule> totalRules, List<BagRule> matchedRules, String color) {
        totalRules.
                forEach(bagRule -> {
                    if (bagRule.containedBags.containsKey(color) &&
                            !matchedRules.contains(bagRule)) {
                        matchedRules.add(bagRule);
                        findMatchingRule(totalRules, matchedRules, bagRule.color);
                    }
                });
    }

    public static long part1(List<BagRule> bagRules) {
        List<BagRule> matchedRules = new ArrayList<>();
        findMatchingRule(bagRules, matchedRules, "shiny gold");
        return matchedRules.size();
    }

    public static BagRule findRule(List<BagRule> totalRules, String color) {
        // Find the rule for the given color
        for (BagRule r : totalRules) {
            if (r.color.equals((color))) {
                return r;
            }
        }
        return null;
    }

    public static int countChildrenBags(List<BagRule> totalRules, String color) {
        BagRule targetRule = findRule(totalRules, color);
        if (targetRule == null) {
            return 0;
        }
        Set<String> colors = targetRule.containedBags.keySet();
        if (colors.isEmpty()) {
            return 0;
        }
        int totalBags = 0;
        for (String inner_bag_color : colors) {
            int inner_bag_count = targetRule.containedBags.get(inner_bag_color);
            totalBags = totalBags + (inner_bag_count) + (inner_bag_count * countChildrenBags(totalRules, inner_bag_color));
        }
        return totalBags;
    }

    public static long part2(List<BagRule> bagRules) {
        return countChildrenBags(bagRules, "shiny gold");
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = Utils.readStringsBySeparator(new File("resources/sample_day_07.txt"), "\n");
        //List<String> lines = Utils.readStringsBySeparator(new File("resources/sample_day_07_1.txt"), "\n");
        List<String> lines = Utils.readStringsBySeparator(new File("resources/input_day_07.txt"), "\n");

        System.out.println("DAY 07 - Handy Haversacks");

        List<BagRule> bagRules = lines.stream().map(BagRule::new).collect(Collectors.toList());
        System.out.println("Part 1: " + part1(bagRules)); // 300
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(bagRules)); // 8030
    }
}
