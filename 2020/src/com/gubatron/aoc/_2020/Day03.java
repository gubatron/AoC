package com.gubatron.aoc._2020;

import java.io.*;
import java.util.ArrayList;
import java.util.List;

public class Day03 {
    public static List<String> readInput(File f) throws IOException {
        List<String> result = new ArrayList<>();
        BufferedReader br = new BufferedReader(new FileReader(f));

        while (br.ready()) {
            String line = br.readLine().strip();
            result.add(line);
        }
        return result;
    }

    public static int part1(String[] forest, int right_moves, int down_moves) {
        int pos_x = 0;
        int pos_y = 0;
        int line_width = forest[0].length();

        int trees = 0;
        while (pos_y < forest.length - 1) {
            pos_x += right_moves;
            pos_x = (pos_x % line_width);

            pos_y += down_moves;

            String line = forest[pos_y];

            if (line.charAt(pos_x) == '#') {
                trees++;
            }
        }
        return trees;
    }

    public static long part2(String[] forest) {
        int[][] slopes = {
                {1, 1},
                {3, 1},
                {5, 1},
                {7, 1},
                {1, 2}
        };
        int[] trees_found = new int[slopes.length];

        long result = 1;
        for (int seq_i = 0; seq_i < slopes.length; seq_i++) {
            trees_found[seq_i] = part1(forest, slopes[seq_i][0], slopes[seq_i][1]);
            System.out.println("Trees @ " + seq_i + " => " + trees_found[seq_i]);
            result = result * trees_found[seq_i];
        }
        return result;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readInput(new File("inputs/sample_day_03.txt"));
        List<String> lines = readInput(new File("inputs/input_day_03.txt"));
        int width = lines.get(0).length();
        int height = lines.size();
        String[] forest = lines.toArray(new String[0]);
        System.out.println("DAY 03");
        System.out.println("Part 1 Trees Found: " + part1(forest, 3, 1));
        System.out.println("\n==============================\n");
        System.out.println("Part 2:\n" + part2(forest));
    }
}
