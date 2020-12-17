package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day16 {
    static class Rule {
        String label;
        int start_1;
        int end_1;
        int start_2;
        int end_2;
        int certain_field_offset = -1;

        Rule(String label_, String range1, String range2) {
            label = label_;
            List<Integer> numbers_range1 = Arrays.stream(range1.split("-")).map(Integer::parseInt).collect(Collectors.toList());
            List<Integer> numbers_range2 = Arrays.stream(range2.split("-")).map(Integer::parseInt).collect(Collectors.toList());
            start_1 = numbers_range1.get(0);
            end_1 = numbers_range1.get(1);
            start_2 = numbers_range2.get(0);
            end_2 = numbers_range2.get(1);
        }

        boolean numberIsValid(int number) {
            return ((start_1 <= number && number <= end_1) ||
                    (start_2 <= number && number <= end_2));
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof Rule)) return false;
            Rule rule = (Rule) o;
            return start_1 == rule.start_1 && end_1 == rule.end_1 && start_2 == rule.start_2 && end_2 == rule.end_2 && Objects.equals(label, rule.label);
        }

        @Override
        public int hashCode() {
            return Objects.hash(label, start_1, end_1, start_2, end_2);
        }
    }

    static class Ticket {
        List<Integer> numbers;
        public boolean isValid = true;

        Ticket(String line) {
            numbers = Arrays.stream(line.split(",")).map(Integer::parseInt).collect(Collectors.toList());
        }
    }

    static class TicketsData {
        List<Rule> rules = new ArrayList<>();
        Ticket ticket = null;
        List<Ticket> nearByTickets;

        boolean readRules;
        boolean readTicket;
        boolean readNearByTickets;

        List<String> buffers = new ArrayList<>();

        public TicketsData(List<String> lines) {
            lines.forEach(line -> {
                if (!readRules && !line.isEmpty()) {
                    buffers.add(line);
                    return;
                } else if (rules.isEmpty() && !readRules) {
                    fillOutRules();
                    readRules = true;
                    return;
                }

                if (!readTicket && !line.equals("your ticket:") && !line.isEmpty()) {
                    ticket = new Ticket(line);
                    readTicket = true;
                    buffers.clear();
                    return;
                }

                if (!readNearByTickets && !line.equals("nearby tickets:") && !line.isEmpty()) {
                    buffers.add(line);
                }
            });

            nearByTickets = buffers.stream().map(Ticket::new).collect(Collectors.toList());
            readNearByTickets = true;
        }

        void fillOutRules() {
            buffers.forEach(line -> {
                        String[] parts = line.split(": ");
                        String[] rules_strings = parts[1].split(" or ");
                        rules.add(new Rule(parts[0], rules_strings[0], rules_strings[1]));
                    }
            );
            buffers.clear();
        }

        int checkTicketInvalidNumbers(Ticket ticket) {
            int ticketErrorRate = 0;
            for (int number : ticket.numbers) {
                boolean numberValid = false;
                for (Rule rule : rules) {
                    if ((rule.start_1 <= number && number <= rule.end_1) ||
                            (rule.start_2 <= number && number <= rule.end_2)) {
                        //System.out.println("Found valid number: " + number + " in rule " + rule.start + "-" + rule.end);
                        numberValid = true;
                        break;
                    }
                }
                if (!numberValid) {
                    ticketErrorRate += number;
                }
            }
            ticket.isValid = ticketErrorRate == 0;
            return ticketErrorRate;
        }

        List<Rule> getDepartureRules() {
            return rules.stream().filter(r -> r.label.startsWith("departure")).collect(Collectors.toList());
        }
    }

    /**
     * To sort the reverse mappings and keep the index of the smallest with more than 1 rule
     */
    static class FieldOffsetRules {
        int field_offset;
        List<Rule> rules;

        FieldOffsetRules(int offset, List<Rule> rs) {
            field_offset = offset;
            rules = rs;
        }

        int size() {
            return rules.size();
        }
    }

    public static long part1(TicketsData ticketsData) {
        return ticketsData.nearByTickets.stream().map(ticketsData::checkTicketInvalidNumbers).reduce(Integer::sum).get();
    }

    static void initializeReverseMap(HashMap<Integer, List<Rule>> reverseMap, List<Rule> rules, List<Ticket> validTickets, final int totalFieldsInTicket) {
        final int totalTickets = validTickets.size();
        HashMap<String, List<Integer>> validityTable = new HashMap<>();

        for (Rule rule : rules) {
            if (!validityTable.containsKey(rule.label)) {
                validityTable.put(rule.label, new ArrayList<>(Collections.nCopies(totalFieldsInTicket, 0)));
            }
        }

        for (Rule rule : rules) {
            if (rule.certain_field_offset != -1) {
                List<Rule> certainRule = new ArrayList<>();
                certainRule.add(rule);
                reverseMap.put(rule.certain_field_offset, certainRule);
                continue;
            }
            // check field by field to add to reverse map
            for (int field_offset = 0; field_offset < totalFieldsInTicket; field_offset++) {
                // All tickets for current field offset
                for (Ticket ticket : validTickets) {
                    int number = ticket.numbers.get(field_offset);
                    if (reverseMap.get(field_offset) != null && reverseMap.get(field_offset).contains(rule)) {
                        continue;
                    }
                    if (rule.certain_field_offset == -1 && rule.numberIsValid(number)) {
                        // chalk one up for this rule and this offset.
                        List<Integer> valid_offset_hits = validityTable.get(rule.label);
                        int previous_hits = valid_offset_hits.get(field_offset);
                        valid_offset_hits.set(field_offset, 1 + previous_hits);
                        if (previous_hits + 1 == totalTickets) {
                            if (!reverseMap.containsKey(field_offset)) {
                                List<Rule> matchingRules = new ArrayList<>();
                                matchingRules.add(rule);
                                reverseMap.put(field_offset, matchingRules);
                            } else {
                                List<Rule> matchingRules = reverseMap.get(field_offset);
                                if (!matchingRules.contains(rule)) {
                                    matchingRules.add(rule);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    static void cleanReverseMapSuspects(List<Ticket> validTickets, List<Rule> rules, HashMap<Integer, List<Rule>> reverseMap) {
        // check the reverse map for eliminations.
        // If one field has a list of only one element
        // We remove that rule from the rules to check.

        int passes = 0;
        while (!allRemainingMappingsHaveOnlyOneValidRule(reverseMap) && passes == 0) {
            int removals = 0;
            Set<Integer> field_offsets = reverseMap.keySet();

            for (int field_offset : field_offsets) {
                List<Rule> matchingRules = reverseMap.get(field_offset);
                if (matchingRules.size() == 1 &&
                        matchingRules.get(0).certain_field_offset == -1) {
                    Rule definiteRule = matchingRules.get(0);
                    // we just found out that this is a definite rule.
                    definiteRule.certain_field_offset = field_offset;
                    for (int i : field_offsets) {
                        if (i != definiteRule.certain_field_offset &&
                                reverseMap.get(i).size() > 1 &&
                                reverseMap.get(i).contains(definiteRule)) {
                            if (reverseMap.get(i).remove(definiteRule)) {
                                removals++;
                                passes = 0;
                            }
                        }
                    }
                }

                if (removals == 0) {
                    passes++;
                }
            }

            List<FieldOffsetRules> rulesSortedBySize = new ArrayList<>();
            for (int field_offset : field_offsets) {
                rulesSortedBySize.add(new FieldOffsetRules(field_offset, reverseMap.get(field_offset)));
            }
            rulesSortedBySize.sort(Comparator.comparingInt(FieldOffsetRules::size));
            FieldOffsetRules smallestFieldOffsetRules = rulesSortedBySize.stream().filter(field_rules -> field_rules.size() > 1).findFirst().orElse(rulesSortedBySize.get(0));
            for (int field_offset : field_offsets) {
                if (field_offset == smallestFieldOffsetRules.field_offset) {
                    // don't delete yourself
                    continue;
                }
                reverseMap.get(field_offset).removeAll(smallestFieldOffsetRules.rules);
            }
        }

    }

    static boolean allRemainingMappingsHaveOnlyOneValidRule(HashMap<Integer, List<Rule>> reverseMap) {
        for (List<Rule> rules : reverseMap.values()) {
            if (rules.size() > 1) return false;
        }
        return true;
    }

    public static long part2(TicketsData ticketsData) {
        ticketsData.nearByTickets.forEach(ticketsData::checkTicketInvalidNumbers);
        List<Ticket> validTickets = ticketsData.nearByTickets.stream().filter(ticket -> ticket.isValid).collect(Collectors.toList());
        validTickets.add(ticketsData.ticket);
        List<Rule> departureRules = ticketsData.getDepartureRules();

        System.out.println("Valid Tickets (including mine): " + validTickets.size());
        System.out.println("Departure Rules: " + departureRules.size());

        // Field offset -> [Rule1, Rule2, ... ]
        List<Rule> rules = new ArrayList<>(ticketsData.rules);
        HashMap<Integer, List<Rule>> reverseMap = new HashMap<>();

        initializeReverseMap(reverseMap, rules, validTickets, ticketsData.ticket.numbers.size());
        cleanReverseMapSuspects(validTickets, rules, reverseMap);

        long result = 1;
        for (int field_offset : reverseMap.keySet()) {
            List<Rule> matchedRules = reverseMap.get(field_offset);
            if (matchedRules.size() == 1 && matchedRules.get(0).label.startsWith("departure")) {
                int number = ticketsData.ticket.numbers.get(field_offset);
                System.out.println(" In my ticket the " + matchedRules.get(0).label + " is field[" + field_offset + "] -> " + number);
                result *= number;
            }
        }
        return result;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_16_1.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_16.txt"), "\n");

        System.out.println("DAY 16 - Ticket Translation");
        TicketsData ticketsData = new TicketsData(lines);

        System.out.println("Part 1: " + part1(ticketsData)); // 19087
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(ticketsData)); // 1382443095281
    }
}
