package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.List;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class DayN {
    public static long part1() {
        return 0;
    }

    public static long part2() {
        return 0;
    }

    public static void main(String[] args) throws IOException {
        List<String> lines = readStringsBySeparator(new File("resources/sample_day_NN.txt"),"\n");
        //List<String> lines = readStringsBySeparator(new File("resources/input_day_NN.txt"),"\n");
        System.out.println("DAY N - ");
        System.out.println("Part 1: " + part1());
        System.out.println("==============================");
        System.out.println("Part 2: " + part2());
    }
}
