package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day20 {
    static class Tile {
        long id;
        List<List<Character>> m;
        int orientation = 0;
        int x;
        int y;

        public Tile(String data) {
            id = Long.parseLong(data.substring(5, 9));
            int i = 10;
            m = new ArrayList<>();
            while (m.size() < 10) {
                List<Character> row = data.substring(i, i + 10).chars().mapToObj(c -> (char) c).collect(Collectors.toList());
                m.add(row);
                i += 10;
            }
            print();
        }

        void rotate() {
            orientation = (orientation + 1) % 4;
            rotate90Clockwise(m);
        }

        void print() {
            System.out.printf("Tile %d\n", id);
            StringBuilder sb = new StringBuilder();
            m.stream().forEach(charList -> {
                charList.forEach(chara -> sb.append(chara));
                System.out.println(sb.toString());
                sb.delete(0, sb.length());
            });
        }
    }

    static void rotate90Clockwise(List<List<Character>> tile) {
        int N = 10;
        // Traverse each cycle
        for (int i = 0; i < N / 2; i++) {
            for (int j = i; j < N - i - 1; j++) {
                // Swap elements of each cycle
                // in clockwise direction
                Character temp = tile.get(i).get(j); //a[i][j];
                //a[i][j] = a[N - 1 - j][i];
                tile.get(i).set(j, tile.get(N - 1 - j).get(i));
                //a[N - 1 - j][i] = a[N - 1 - i][N - 1 - j];
                tile.get(N - 1 - j).set(i, tile.get(N - 1 - i).get(N - 1 - j));
                //a[N - 1 - i][N - 1 - j] = a[j][N - 1 - i];
                tile.get(N - 1 - i).set(N - 1 - j, tile.get(j).get(N - 1 - i));
                //a[j][N - 1 - i] = temp;
                tile.get(j).set(N - 1 - i, temp);
            }
        }
    }

    static List<Tile> loadTiles(List<String> lines) {
        List<Tile> tiles = new ArrayList<>();
           int lineNumber = 0;

        while (lineNumber < lines.size()) {
            StringBuilder sb = new StringBuilder();
            for (int i=lineNumber; i < lineNumber+11; i++) {
                //System.out.printf("%d - %s\n",i,lines.get(i));
                sb.append(lines.get(i));
            }
            lineNumber+=12;
            tiles.add(new Tile(sb.toString()));
            //System.out.println();
        }
        return tiles;
    }

    public static long part1() {
        return 0;
    }

    public static long part2() {
        return 0;
    }

    public static void main(String[] args) throws IOException {
        List<String> lines = readStringsBySeparator(new File("resources/sample_day_20.txt"), "\n");
        List<Tile> tiles = loadTiles(lines);

        tiles.get(0).print();
        tiles.get(0).rotate();
        tiles.get(0).print();
        //List<String> lines = readStringsBySeparator(new File("resources/input_day_20.txt"),"\n");
        System.out.println("DAY 20 - Jurassic Jigsaw");
        System.out.println("Part 1: " + part1());
        System.out.println("==============================");
        System.out.println("Part 2: " + part2());
    }
}
