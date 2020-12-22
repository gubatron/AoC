package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Stack;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day22 {

    static class CrabCombat {
        ArrayList<Integer> player1;
        ArrayList<Integer> player2;

        public CrabCombat(List<String> lines) {
            player1 = new ArrayList<>();
            player2 = new ArrayList<>();
            lines.stream().takeWhile(s -> !s.isEmpty()).skip(1).mapToInt(Integer::parseInt).forEach(player1::add);
            lines.stream().skip(3 + player1.size()).mapToInt(Integer::parseInt).forEach(player2::add);
            System.out.println("Check the stacks.");
        }

        public void run() {
            int round = 1;
            while (player1.size() > 0 && player2.size() > 0) {
                System.out.printf("-- Round %d --\n", round++);
                printDeck(1, player1);
                printDeck(2, player2);

                int p1Card = player1.remove(0);
                int p2Card = player2.remove(0);
                System.out.printf("Player 1's plays: %d\n", p1Card);
                System.out.printf("Player 2's plays: %d\n", p2Card);

                if (p1Card > p2Card) {
                    System.out.println("Player 1 wins the round!\n");
                    player1.add(p1Card);
                    player1.add(p2Card);
                } else {
                    System.out.println("Player 2 wins the round!\n");
                    player2.add(p2Card);
                    player2.add(p1Card);
                }
            }
            System.out.println("\n== Post-game results ==");
            printDeck(1, player1);
            printDeck(2, player2);
        }

        void printDeck(int playerNumber, ArrayList<Integer> cards) {
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < cards.size(); i++) {
                sb.append(cards.get(i));
                if (i < cards.size() - 1) {
                    sb.append(", ");
                }
            }
            System.out.printf("Player's %d deck: %s\n", playerNumber, sb.toString());
        }

        long printScore() {
            ArrayList<Integer> cards = player1.size() > player2.size() ? player1 : player2;
            int maxMultiplier = cards.size();
            boolean firstLine = true;
            long result = 0;
            for (Integer card : cards) {
                if (firstLine) {
                    System.out.printf("  %2d * %2d\n", card, maxMultiplier);
                } else {
                    System.out.printf("+ %2d * %2d\n", card, maxMultiplier);
                }
                result += card * maxMultiplier--;
            }
            System.out.printf("=%d\n", result);
            return result;
        }
    }

    public static long part1(CrabCombat crabCombat) {
        crabCombat.run();
        return crabCombat.printScore();
    }

    public static long part2(CrabCombat crabCombat) {
        return 0;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_22.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_22.txt"), "\n");
        CrabCombat crabCombat = new CrabCombat(lines);
        System.out.println("DAY 22 - Crab Combat");
        System.out.println("Part 1: " + part1(crabCombat)); // 34566
        System.out.println("==============================");
        crabCombat = new CrabCombat(lines);
        System.out.println("Part 2: " + part2(crabCombat));
    }
}
