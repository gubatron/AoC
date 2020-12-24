package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day22 {

    static class CrabCombat {
        ArrayList<Integer> player1;
        ArrayList<Integer> player2;

        ArrayList<String> p1PrevStates;
        ArrayList<String> p2PrevStates;

        public CrabCombat(List<String> lines) {
            player1 = new ArrayList<>();
            player2 = new ArrayList<>();
            p1PrevStates = new ArrayList<>();
            p2PrevStates = new ArrayList<>();
            lines.stream().takeWhile(s -> !s.isEmpty()).skip(1).mapToInt(Integer::parseInt).forEach(player1::add);
            lines.stream().skip(3 + player1.size()).mapToInt(Integer::parseInt).forEach(player2::add);
        }

        public CrabCombat(ArrayList<Integer> player1, ArrayList<Integer> player2) {
            this.player1 = player1;
            this.player2 = player2;
            p1PrevStates = new ArrayList<>();
            p2PrevStates = new ArrayList<>();
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

        public static int runRecursive(int gameNumber, CrabCombat cc) {
            System.out.printf("=== Game %d ===\n", gameNumber);
            int round = 1;
            while (cc.player1.size() > 0 && cc.player2.size() > 0) {
                System.out.printf("\n-- Round %d (Game %d)--\n", round, gameNumber);
                printDeck(1, cc.player1);
                printDeck(2, cc.player2);

                cc.p1PrevStates.add(deckToString(cc.player1));
                cc.p2PrevStates.add(deckToString(cc.player2));

                int p1Card = cc.player1.remove(0);
                int p2Card = cc.player2.remove(0);

                System.out.printf("Player 1's plays: %d\n", p1Card);
                System.out.printf("Player 2's plays: %d\n", p2Card);

                if (cc.p1PrevStates.size() > 0) {
                    if (cc.p1PrevStates.contains(deckToString(cc.player1))) {
                        System.out.println("Repeated state for player 1, player 1 wins");
                        System.out.println(" -> " + deckToString(cc.player1));
                        System.out.printf("The winner of game %d is player %d!", gameNumber, 1);
                        if (gameNumber > 1) {
                            System.out.printf("\n...anyway back to game %d.\n\n", (gameNumber-1));
                        }
                        return 1;
                    }
                    if (cc.p2PrevStates.contains(deckToString(cc.player2))) {
                        System.out.println("Repeated state for player 2, player 1 wins");
                        System.out.println(" -> " + deckToString(cc.player2));
                        System.out.printf("The winner of game %d is player %d!", gameNumber, 1);
                        if (gameNumber > 1) {
                            System.out.printf("\n...anyway back to game %d.\n\n", (gameNumber-1));
                        }
                        return 1;
                    }
                }

                int roundWinner;
                if (p1Card <= cc.player1.size() && p2Card <= cc.player2.size()) {
                    System.out.println("Playing a sub-game to determine the winner...");
                    CrabCombat crabCombat = new CrabCombat(new ArrayList<>(cc.player1.subList(0, p1Card)),
                            new ArrayList<>(cc.player2.subList(0, p2Card)));
                    roundWinner = runRecursive(gameNumber+1, crabCombat);
                } else {
                    roundWinner = (p1Card > p2Card) ? 1 : 2;
                }

                if (roundWinner == 1) {
                    System.out.printf("Player 1 wins round %d of game %d!\n", round, gameNumber);
                    cc.player1.add(p1Card);
                    cc.player1.add(p2Card);
                } else if (roundWinner == 2) {
                    System.out.printf("Player 2 wins round %d of game %d!\n", round, gameNumber);
                    cc.player2.add(p2Card);
                    cc.player2.add(p1Card);
                }
                round++;
            }
            int gameWinner = cc.player1.size() > cc.player2.size() ? 1 : 2;
            System.out.printf("The winner of game %d is player %d!", gameNumber, gameWinner);
            if (gameNumber > 1) {
                System.out.printf("\n...anyway back to game %d.\n\n", (gameNumber-1));
            }
            return gameWinner;
        }

        static String deckToString(ArrayList<Integer> cards) {
            if (cards.isEmpty()) return "";
            String deckS = cards.stream().map(String::valueOf).map(s -> s + ", ").reduce(String::concat).get();
            return deckS.substring(0, deckS.lastIndexOf(","));
        }

        static void printDeck(int playerNumber, ArrayList<Integer> cards) {
            System.out.printf("Player's %d deck: %s\n", playerNumber, deckToString(cards));
        }

        long printScore(ArrayList<Integer> cards) {
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
        ArrayList<Integer> winningCards =
                crabCombat.player1.size() > crabCombat.player2.size() ? crabCombat.player1 : crabCombat.player2;
        return crabCombat.printScore(winningCards);
    }

    public static long part2(CrabCombat crabCombat) {
        int winner = CrabCombat.runRecursive(1, crabCombat);

        System.out.println("\n== Post-game results ==");
        CrabCombat.printDeck(1, crabCombat.player1);
        CrabCombat.printDeck(2, crabCombat.player2);

        ArrayList<Integer> winningCards =
                crabCombat.player1.size() > crabCombat.player2.size() ? crabCombat.player1 : crabCombat.player2;
        return crabCombat.printScore(winningCards);
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
