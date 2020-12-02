package com.gubatron.aoc._2020;

import java.io.*;
import java.util.ArrayList;
import java.util.List;

public class Day03 {
    static class Something {

    }
    public static List<Something> readInput(File f) throws IOException {
        List<Something> result = new ArrayList<>();
        BufferedReader br = new BufferedReader(new FileReader(f));
        while (br.ready()) {
            String line = br.readLine();
            //
            //result.add(pp);
        }
        return result;
    }

    public void part1(List<Something> data) {

    }

    public void part2(List<Something> data) {

    }

    public static void main(String[] args) throws IOException {
        List<Something> data = readInput(new File("sample_day_03.txt"));
        //List<Something> data = readInput(new File("input_day_03.txt"));
    }
}
