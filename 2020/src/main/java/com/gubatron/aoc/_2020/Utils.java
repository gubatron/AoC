package com.gubatron.aoc._2020;

import java.io.*;
import java.util.ArrayList;
import java.util.List;

public final class Utils {
    static List<String> readStringList(File f) throws IOException {
        List<String> result = new ArrayList<>();
        BufferedReader br = new BufferedReader(new FileReader(f));

        while (br.ready()) {
            String line = br.readLine().strip();
            result.add(line);
        }
        return result;
    }

    public static List<Integer> readIntList(File f) throws FileNotFoundException {
        List<Integer> list = new ArrayList<>();
        BufferedReader br = new BufferedReader(new FileReader(f));
        while (true) {
            try {
                if (!br.ready()) break;
            } catch (IOException e) {
                e.printStackTrace();
            }
            try {
                list.add(Integer.parseInt(br.readLine()));
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
        return list;
    }
}
