package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day20 {
    enum Side {
        TOP,
        RIGHT,
        BOTTOM,
        LEFT
    }

    static class Tile {
        long id;
        List<List<Character>> m;
        List<String> sides;
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
            sides();
        }

        void rotate() {
            System.out.printf("Rotating Tile %d\n", id);
            orientation = (orientation + 1) % 4;
            rotate90Clockwise(m);
        }

        String side(Side s) {
            StringBuilder sb = new StringBuilder();
            if (s == Side.TOP) {
                m.get(0).forEach(sb::append);
                return sb.toString();
            }
            if (s == Side.BOTTOM) {
                m.get(9).forEach(sb::append);
            }
            if (s == Side.RIGHT) {
                m.forEach(char_list -> sb.append(char_list.get(9)));
            }
            if (s == Side.LEFT) {
                m.forEach(char_list -> sb.append(char_list.get(0)));
            }
            return sb.toString();
        }

        List<String> sides() {
            if (sides == null) {
                sides = new ArrayList<>();
                sides.add(side(Side.TOP));
                sides.add(side(Side.RIGHT));
                sides.add(side(Side.BOTTOM));
                sides.add(side(Side.LEFT));
            }
            return sides;
        }

        void print() {
            System.out.printf("Tile %d\n", id);
            StringBuilder sb = new StringBuilder();
            m.forEach(charList -> {
                charList.forEach(sb::append);
                System.out.println(sb.toString());
                sb.delete(0, sb.length());
            });
        }

        int sidesMatches(Tile other) {
            int matches = 0;
            List<String> otherSides = other.sides();
            for (String mySide : sides) {
                for (String theirSide : otherSides) {
                    if (mySide.equals(theirSide) || reverseString(theirSide).equals(mySide) ||
                            reverseString(mySide).equals(theirSide) || reverseString(mySide).equals(reverseString(theirSide))) {
                        //System.out.println("\t" + mySide + " vs " + theirSide + " (MATCH)");
                        matches++;
                    }
                }
            }
            return matches;
        }
    }

    static class TileBoard {
        Map<Long, Set<Long>> matches;
        Map<Long, Tile> idMap;

        public TileBoard(List<Tile> tiles) {
            int N = (int) Math.sqrt(tiles.size());
            Tile[][] board = new Tile[N][N];
            idMap = new HashMap<>();
            matches = new HashMap<>();

            tiles.forEach(tile -> {
                idMap.put(tile.id, tile);
                matches.put(tile.id, new HashSet<>());
            });
        }

        void matchTiles() {
            idMap.keySet().forEach(this::findMatches);
            System.out.println("========================================");
            idMap.keySet().forEach(tileID -> System.out.printf("Tile %d has %d matches\n", tileID, matches.get(tileID).size()));
            List<Tile> unmatched = matches.keySet().stream().filter(id -> matches.get(id).size() == 0).map(id -> idMap.get(id)).collect(Collectors.toList());
            if (unmatched.size() > 0) {
                // Rotate all unmatched and try finding their matches
                unmatched.forEach(Tile::rotate);
                matchTiles();
            }
        }

        void findMatches(long tileId) {
            Tile myTile = idMap.get(tileId);
            idMap.keySet().forEach(otherId -> {
                if (otherId != tileId) {
                    Tile other = idMap.get(otherId);
                    int matchedSides = myTile.sidesMatches(other);
                    Set<Long> matchesForMyTile = matches.get(tileId);
                    Set<Long> matchesForOtherTile = matches.get(otherId);
                    if (matchedSides > 0) {
                        System.out.printf("Tile %d matched %d sides with Tile %d\n", myTile.id, matchedSides, otherId);
                        matchesForMyTile.add(otherId);
                        matchesForOtherTile.add(tileId);
                    } else {
                        // We need to rotate and try again :)
                        System.out.printf("Tile %d had no matches\n", tileId);
                    }
                }
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
            for (int i = lineNumber; i < lineNumber + 11; i++) {
                //System.out.printf("%d - %s\n",i,lines.get(i));
                sb.append(lines.get(i));
            }
            lineNumber += 12;
            tiles.add(new Tile(sb.toString()));
            //System.out.println();
        }
        return tiles;
    }

    static String reverseString(String s) {
        StringBuilder sb = new StringBuilder();
        for (int i = s.length() - 1; i >= 0; i--) sb.append(s.charAt(i));
        return sb.toString();
    }

    static long part1Result = 1;

    public static long part1(List<Tile> tiles) {
        TileBoard board = new TileBoard(tiles);
        board.matchTiles(); // match all possible combinations of sides
        board.matches.entrySet().stream().
                filter(longSetEntry -> longSetEntry.getValue().size() == 2). //those tiles that have only 2 matches, are corners
                map(longSetEntry -> board.idMap.get(longSetEntry.getKey())).forEach(tile -> part1Result *= tile.id);
        return part1Result;
    }

    public static long part2() {
        return 0;
    }

    public static void main(String[] args) throws IOException {
        System.out.println("Loading tiles...");
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_20.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_20.txt"), "\n");
        List<Tile> tiles = loadTiles(lines);
        System.out.printf("Loaded %d tiles.\n", tiles.size());
        System.out.println("DAY 20 - Jurassic Jigsaw");
        System.out.println("Part 1: " + part1(tiles)); //84116744709593 / 639ms
        System.out.println("==============================");
        System.out.println("Part 2: " + part2());
    }
}
