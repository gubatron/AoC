package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day19 {
    static class Rule {
        String val;
        // inner lists are to be collapsed into string expressions, each list can be a match as an OR
        List<List<Integer>> ruleIds;
        String originalString;
        boolean resolved;
        // Each set of rules should translate to an expression
        List<String> expressions;
        boolean reference = false;

        public Rule(int ruleId) {
            initFromRuleID(ruleId);
        }

        public void initFromRuleID(int ruleId) {
            originalString = String.format("@%d", ruleId);
            List<List<Rule>> rules = new ArrayList<>();
            ruleIds = new ArrayList<>();
            List<Integer> ids = new ArrayList<>();
            ids.add(ruleId);
            ruleIds.add(ids);
            resolved = false;
            reference = true;
        }

        public Rule(String line) {
            if (line.contains("|")) {
                resolved = false;
                ruleIds = new ArrayList<>();
                String[] ors = line.split(" \\| ");
                List<Integer> leftSideIds = Arrays.stream(ors[0].strip().split(" ")).map(Integer::parseInt).collect(Collectors.toList());
                List<Integer> rightSideIds = Arrays.stream(ors[1].strip().split(" ")).map(Integer::parseInt).collect(Collectors.toList());
                ruleIds.add(leftSideIds);
                ruleIds.add(rightSideIds);
            } else {
                // Literal Char "a" or "b"
                if (line.startsWith("\"")) {
                    val = line.replace("\"", "");
                    expressions = new ArrayList<>();
                    expressions.add(val);
                    resolved = true;
                }
                // only one set of rules n1 n2 n3 ...
                else {
                    resolved = false;
                    ruleIds = new ArrayList<>();
                    List<Integer> andRuleIds = Arrays.stream(line.split(" ")).map(Integer::parseInt).collect(Collectors.toList());
                    ruleIds.add(andRuleIds);
                }
            }
            originalString = line;
        }

        void resolveExpressions(Map<Integer, Rule> ruleMap) {
            if (resolved) {
                return;
            }
            if (expressions == null) {
                expressions = new ArrayList<>();
            }

            for (List<Integer> andRuleGroup : ruleIds) {
                StringBuilder expressionBuilder = new StringBuilder();
                for (Integer ruleId : andRuleGroup) {
                    Rule rule = ruleMap.get(ruleId);
                    if (rule.resolved && rule.val != null) {
                        expressionBuilder.append(rule.val);
                    } else {
                        if (!rule.resolved) {
                            rule.resolveExpressions(ruleMap);
                        }
                    }
//                    for (String exp : rule.expressions) {
//                        expressionBuilder.append(exp);
//                    }
                }
                if (expressionBuilder.length() > 0) {
                    expressions.add(expressionBuilder.toString());
                }
            }
            resolved = true;
        }

        @Override
        public String toString() {
            return "Rule{" +
                    ", val='" + val + '\'' +
                    ", ruleIds=" + ((ruleIds == null) ? "<literal>" : ruleIds.size()) +
                    ", originalString='" + originalString + '\'' +
                    ", resolved=" + resolved +
                    ", expressions=" + (expressions == null ? "<unresolved>" : expressions.stream().reduce((s, s2) -> s + s2).get()) +
                    '}';
        }
    }

    static class Input {
        int index = 0;
        Map<Integer, Rule> ruleMap;
        List<Rule> rules;
        List<String> messages;

        public Input(final List<String> lines) {
            ruleMap = new HashMap<>();
            rules = lines.stream().takeWhile(line -> !line.isEmpty()).map(this::onRuleLine).collect(Collectors.toList());
            messages = lines.stream().skip(rules.size() + 1).collect(Collectors.toList());
        }

        Rule onRuleLine(String line) {
            String[] splitRule = line.split(": ");
            Rule rule = new Rule(splitRule[1]);
            ruleMap.put(Integer.parseInt(splitRule[0]), rule);
            return rule;
        }
    }

    public static long part1(Input input) {
        input.ruleMap.get(0).resolveExpressions(input.ruleMap);

        System.out.println("Valid Expressions for Rule 0");
        var ref = new Object() {
            public int val = 0;
        };
        input.ruleMap.get(0).expressions.forEach(exp -> {
            System.out.println("- " + ref.val + " " + exp);
            ref.val++;
        });
        return 0;
    }

    public static long part2() {
        return 0;
    }

    public static void main(String[] args) throws IOException {
        List<String> lines = readStringsBySeparator(new File("resources/sample_day_19.txt"), "\n");
        //List<String> lines = readStringsBySeparator(new File("resources/input_day_19.txt"), "\n");
        Input input = new Input(lines);
        System.out.println("DAY 19 - Monster Messages");
        System.out.println("Part 1: " + part1(input));
        System.out.println("==============================");
        System.out.println("Part 2: " + part2());
    }
}
