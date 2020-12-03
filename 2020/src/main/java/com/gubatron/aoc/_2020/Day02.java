package com.gubatron.aoc._2020;

import java.io.*;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day02 {
    static final Pattern policy_pass_pattern = Pattern.compile("([0-9]+)-([0-9]+) ([a-z]+): ([a-z]+)");

    static class PolicyPass {
        int min;
        int max;
        char c;
        String pass;
        String line;
    }

    public static PolicyPass build(String line) {
        Matcher matcher = policy_pass_pattern.matcher(line);
        if (matcher.find()) {
            PolicyPass pp = new PolicyPass();
            pp.min = Integer.parseInt(matcher.group(1));
            pp.max = Integer.parseInt(matcher.group(2));
            pp.c = matcher.group(3).charAt(0);
            pp.pass = matcher.group(4);
            pp.line = line;
            return pp;
        }
        return null;
    }

    public static boolean isPasswordValid(PolicyPass pp) {
        int count = (int) pp.pass.chars().filter(ch -> ch == pp.c).count();
        return count >= pp.min && count <= pp.max;
    }

    public static boolean isPasswordValid2(PolicyPass pp) {
        int index_1 = pp.min - 1;
        int index_2 = pp.max - 1;
        return (pp.pass.charAt(index_1) == pp.c &&
                pp.pass.charAt(index_2) != pp.c) ||
                (pp.pass.charAt(index_1) != pp.c &&
                        pp.pass.charAt(index_2) == pp.c);
    }

    public static List<PolicyPass> readInput(File f) throws IOException {
        List<PolicyPass> result = new ArrayList<>();
        BufferedReader br = new BufferedReader(new FileReader(f));
        while (br.ready()) {
            String line = br.readLine();
            PolicyPass pp = build(line);
            result.add(pp);
        }
        return result;
    }

    public static int part1(List<PolicyPass> list) {
        int validCount = 0;
        for (PolicyPass pp : list) {
            if (isPasswordValid(pp)) {
                validCount++;
            }
        }
        return validCount;
    }

    public static int part2(List<PolicyPass> list) {
        int validCount = 0;
        for (PolicyPass pp : list) {
            if (isPasswordValid2(pp)) {
                validCount++;
            }
        }
        return validCount;
    }


    public static void main(String[] args) throws IOException {
        List<PolicyPass> policyPasses = readInput(new File("resources/input_day_02.txt"));
        //List<PolicyPass> policyPasses = readInput(new File("resources/sample_day_02.txt"));
        System.out.println("Part 1: " + part1(policyPasses));
        System.out.println("Part 2: " + part2(policyPasses));
    }
}
