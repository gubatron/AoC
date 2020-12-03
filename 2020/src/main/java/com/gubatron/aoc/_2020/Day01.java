package com.gubatron.aoc._2020;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.List;

import static com.gubatron.aoc._2020.Utils.readIntList;

public class Day01 {
    public static int partOne(List<Integer> list) {
        for (int i = 0; i < list.size(); i++) {
            for (int j = 0; j < list.size(); j++) {
                if (j == i) {
                    j++;
                    continue;
                }
                int a = list.get(i);
                int b = list.get(j);

                if (a + b == 2020) {
                    System.out.println("a=" + a);
                    System.out.println("b=" + b);
                    return a * b;
                }
            }
        }

        return 0;
    }

    public static int partTwo(List<Integer> list) {
        for (int i = 0; i < list.size(); i++) {
            for (int j = 0; j < list.size(); j++) {
                for (int k = 0; k < list.size(); k++) {
                    if (j == k || k == i || j == i) {
                        continue;
                    }
                    int a = list.get(i);
                    int b = list.get(j);
                    int c = list.get(k);

                    if ((a + b + c) == 2020) {
                        System.out.println("a=" + a);
                        System.out.println("b=" + b);
                        System.out.println("c=" + c);
                        return a * b * c;
                    }
                }
            }
        }

        return 0;
    }

    public static void main(String[] args) throws FileNotFoundException {
        File f = new File("resources/input_day_01.txt");
        List<Integer> list = readIntList(f);
        System.out.println("Part 1:");
        System.out.println(partOne(list) + "\n");

        System.out.println("Part 2:");
        System.out.println(partTwo(list));
    }
}

