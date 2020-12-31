package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.Arrays;
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
                        } else {
                            //nw
                            y++;
                        }
                        z--;
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
                        } else {
                            //sw
                            x--;
                        }
                        z++;
                    }
                    break;
            }
        }
        return new Coordinate(x, y, z);
    }

    private static void initialFlip(Stream<Coordinate> coordinateStream, HashMap<Coordinate, Boolean> tiles) {
        coordinateStream.forEach(coord -> {
            if (tiles.containsKey(coord)) {
                tiles.put(coord, !tiles.get(coord));
            } else {
                tiles.put(coord, true); // black == true
            }
        });
    }

    private static long countBlackTiles(HashMap<Coordinate, Boolean> tiles) {
        return tiles.values().stream().filter(b -> b).count();
    }

    public static long part1(Stream<Coordinate> coordinateStream) {
        HashMap<Coordinate, Boolean> tiles = new HashMap<>();
        initialFlip(coordinateStream, tiles);
        return countBlackTiles(tiles);
    }

    private static int countBlackNbrs(Coordinate coord, HashMap<Coordinate, Boolean> tiles, HashMap<Coordinate, Boolean> newTiles) {
        int blacks = 0;
        int[][] deltas = new int[][]{
                new int[]{1, -1, 0},
                new int[]{0, -1, 1},
                new int[]{-1, 0, 1},
                new int[]{-1, 1, 0},
                new int[]{0, 1, -1},
                new int[]{1, 0, -1},
        };

        for (int[] delta : deltas) {
            Coordinate nbr = new Coordinate(coord.x + delta[0], coord.y + delta[1], coord.z + delta[2]);
            if (tiles.containsKey(nbr)) {
                if (tiles.get(nbr)) {
                    blacks++;
                }
            } else {
                newTiles.put(nbr, false);
            }
        }
        return blacks;
    }

    private static void addNewNbrs(Coordinate coord,
                                   HashMap<Coordinate, Boolean> tiles,
                                   HashMap<Coordinate, Boolean> newTiles) {
        int blacks = 0;
        int[][] deltas = new int[][]{
                new int[]{1, -1, 0},
                new int[]{0, -1, 1},
                new int[]{-1, 0, 1},
                new int[]{-1, 1, 0},
                new int[]{0, 1, -1},
                new int[]{1, 0, -1},
        };

        Arrays.stream(deltas).forEach(delta -> {
            Coordinate nbr = new Coordinate(coord.x + delta[0], coord.y + delta[1], coord.z + delta[2]);
            if (!tiles.containsKey(nbr)) {
                newTiles.put(nbr, false);
            }
        });
    }

    public static long part2(Stream<Coordinate> coordinateStream) {
        HashMap<Coordinate, Boolean> tiles = new HashMap<>();
        initialFlip(coordinateStream, tiles);
        for (int day = 1; day <= 100; day++) {
            HashMap<Coordinate, Boolean> flips = new HashMap<>();
            final HashMap<Coordinate, Boolean> newTiles = new HashMap<>();
            tiles.forEach((tile,isBlack) -> addNewNbrs(tile,tiles,newTiles));
            newTiles.forEach(tiles::put);
            newTiles.clear();

            tiles.forEach((coord, isBlack) -> {
                int numBlackNbrs = countBlackNbrs(coord, tiles, newTiles);
                if (isBlack && (numBlackNbrs == 0 || numBlackNbrs > 2)) {
                    flips.put(coord, false);
                } else if (!isBlack && numBlackNbrs == 2) {
                    flips.put(coord, true);
                }
            });

            // flip em
            flips.forEach(tiles::put);
            if (day <= 10 || (((day) % 10) == 0)) {
                System.out.printf("Day %d: %d\n", day, countBlackTiles(tiles));
            }
        }
        return countBlackTiles(tiles);
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_24.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_24.txt"), "\n");
        System.out.println("DAY 24 - Lobby Layout");
        long start_t = System.currentTimeMillis();
        Stream<Coordinate> coordinateStream = lines.stream().map(Day24::lineToCoordinate);
        System.out.println("Part 1: " + part1(coordinateStream));  // 228
        long end_t = System.currentTimeMillis() - start_t;
        System.out.printf("%d ms\n", end_t);
        System.out.println("==============================");

        coordinateStream = lines.stream().map(Day24::lineToCoordinate);
        start_t = System.currentTimeMillis();
        System.out.println("Part 2: " + part2(coordinateStream)); // 3672
        end_t = System.currentTimeMillis() - start_t;
        System.out.printf("%d ms\n", end_t);
    }
}
