package com.gubatron.aoc._2020;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class Day04 {
    public static List<String> readInput(File f) throws IOException {
        List<String> result = new ArrayList<>();
        BufferedReader br = new BufferedReader(new FileReader(f));

        while (br.ready()) {
            String line = br.readLine().strip();
            result.add(line);
        }
        return result;
    }

    public static long part1() {
        return 0;
    }

    public static long part2() {
        return 0;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readInput(new File("resources/sample_day_04.txt"));
        List<String> lines = readInput(new File("resources/input_day_04.txt"));
        System.out.println("DAY 04");
        System.out.println("Part 1" + part1());
        System.out.println("\n==============================\n");
        System.out.println("Part 2:\n" + part2());
    }
}
