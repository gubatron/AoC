package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Objects;
import java.util.stream.Stream;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day24 {

    static class Coordinate {
        final int x;
        final int y;
        final int z;

        Coordinate(int x, int y, int z) {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof Coordinate)) return false;
            Coordinate that = (Coordinate) o;
            return x == that.x && y == that.y && z == that.z;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y, z);
        }

        @Override
        public String toString() {
            return "Coordinate{" +
                    "x=" + x +
                    ", y=" + y +
                    ", z=" + z +
                    '}';
        }
    }

    static Coordinate lineToCoordinate(String line) {
        int x = 0;
        int y = 0;
        int z = 0;
        int i = 0;
        int line_length = line.length();
        while (i < line_length) {
            char c = line.charAt(i++);
            switch (c) {
                case 'e':
                    x++;
                    y--;
                    break;
                case 'w':
                    x--;
                    y++;
                    break;
                case 'n':
                    if (i < line_length) {
                        if (line.charAt(i++) == 'e') {
                            //ne
                            x++;
                            z--;
                        } else {
                            //nw
                            y++;
                            z--;
                        }
                    } else {
                        System.out.println(line);
                        throw new RuntimeException();
                    }
                    break;
                case 's':
                    if (i < line_length) {
                        if (line.charAt(i++) == 'e') {
                            //se
                            y--;
                            z++;
                        } else {
                            //sw
                            x--;
                            z++;
                        }
                    }
                    break;
            }
        }
        return new Coordinate(x, y, z);
    }

    public static long part1(Stream<Coordinate> coordinateStream) {
        HashMap<Coordinate, Boolean> tiles = new HashMap<>();
        coordinateStream.forEach(coord -> {
            if (tiles.containsKey(coord)) {
                tiles.put(coord, !tiles.get(coord));
            } else {
                tiles.put(coord, false);
            }
        });
        return tiles.values().stream().filter(b -> !b).count();
    }

    public static long part2(Stream<Coordinate> coordinateStream) {
        return 0;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_24.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_24.txt"), "\n");
        System.out.println("DAY 24 - Lobby Layout");
        long start_t = System.currentTimeMillis();
        Stream<Coordinate> coordinateStream = lines.stream().map(Day24::lineToCoordinate);
        System.out.println("Part 1: " + part1(coordinateStream));
        long end_t = System.currentTimeMillis() - start_t;
        System.out.printf("%d ms\n", end_t); // 228
        System.out.println("==============================");
        start_t = System.currentTimeMillis();
        System.out.println("Part 2: " + part2(coordinateStream));
        end_t = System.currentTimeMillis() - start_t;
        System.out.printf("%d ms\n", end_t); // 228
    }
}
