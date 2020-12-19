package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.stream.Stream;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day17 {

    static final class Cube {
        private final Coordinate c;
        private boolean active;
        private boolean willFlip;

        Cube(Coordinate c, boolean state) {
            this.c = c;
            active = state;
        }

        Stream<Cube> neighbors3D(PocketDimension pd) {
            List<Cube> nbrs = new ArrayList<>();
            final Integer[] deltas = new Integer[]{-1, 0, 1};
            for (int dx : deltas) {
                for (int dy : deltas) {
                    for (int dz : deltas) {
                        // don't consider yourself a neighbor
                        if (dx == 0 && dy == 0 && dz == 0) {
                            continue;
                        }
                        Cube cube = pd.getCube(c.x + dx, c.y + dy, c.z + dz);
                        nbrs.add(cube);
                    }
                }
            }
            return nbrs.stream();
        }

        private int getNumActiveNeighbors3D(PocketDimension pocketDimension) {
            return (int) neighbors3D(pocketDimension).filter(c -> c.active).count();
        }

        Stream<Cube> neighbors4D(PocketDimension pd) {
            List<Cube> nbrs = new ArrayList<>();
            Integer[] deltas = new Integer[]{-1, 0, 1};
            for (int dx : deltas) {
                for (int dy : deltas) {
                    for (int dz : deltas) {
                        for (int dw : deltas) {
                            // don't consider yourself a neighbor
                            if (dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                                continue;
                            }
                            nbrs.add(pd.getHyperCube(c.x + dx, c.y + dy, c.z + dz, c.w + dw));
                        }
                    }
                }
            }
            return nbrs.stream();
        }

        int getNumActiveNeighbors4D(PocketDimension pocketDimension) {
            return (int) neighbors4D(pocketDimension).filter(c -> c.active).count();
        }

        private void preCycle3D(PocketDimension pocketDimension) {
            int numActiveNeighbors = getNumActiveNeighbors3D(pocketDimension);
            //KISS
            if (active) {
                willFlip = numActiveNeighbors != 2 && numActiveNeighbors != 3;
            } else {
                willFlip = numActiveNeighbors == 3;
            }
        }

        private void preCycle4D(PocketDimension pocketDimension) {
            int numActiveNeighbors = getNumActiveNeighbors4D(pocketDimension);
            //KISS
            if (active) {
                willFlip = numActiveNeighbors != 2 && numActiveNeighbors != 3;
            } else {
                willFlip = numActiveNeighbors == 3;
            }
        }

        // Applies the active flip, should be called only after every Cube has pre-cycled
        void cycle() {
            if (willFlip) {
                active = !active;
                willFlip = false;
            }
        }

        public void setActive(boolean act) {
            active = act;
        }
    }

    static final class Coordinate {
        int x;
        int y;
        int z;
        int w;

        public Coordinate(int x, int y, int z, int w) {
            this.x = x;
            this.y = y;
            this.z = z;
            this.w = w;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof Coordinate)) return false;
            Coordinate that = (Coordinate) o;
            return x == that.x && y == that.y && z == that.z && w == that.w;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y, z, w);
        }
    }


    static class PocketDimension {
        private final Map<Coordinate, Cube> matrix = new HashMap<>();
        private final Map<Coordinate, Cube> tempNewCubes = new HashMap<>();

        PocketDimension(List<String> lines) {
            int y = 0;
            for (String line : lines) {
                char[] chars = line.toCharArray();
                for (int x = 0; x < chars.length; x++) {
                    Coordinate coordinate = new Coordinate(x, y, 0, 0);
                    matrix.put(coordinate, createCube(coordinate, chars[x] == '#'));
                }
                y++;
            }
        }

        int getNumActiveCubes() {
            return (int) matrix.values().stream().filter(cube -> cube.active).count();
        }

        public void cycle3D() {
            tempNewCubes.clear();
            matrix.values().forEach(cube -> cube.neighbors3D(this));
            tempNewCubes.forEach(matrix::put);
            tempNewCubes.clear();
            matrix.values().forEach(cube -> cube.preCycle3D(this));
            matrix.values().forEach(Cube::cycle);
        }

        public Cube createCube(Coordinate coordinate, boolean active) {
            Cube cube = new Cube(coordinate, active);
            tempNewCubes.put(coordinate, cube);
            return cube;
        }

        Cube getCube(int x, int y, int z) {
            return getHyperCube(x, y, z, 0);
        }

        public Cube getHyperCube(int x, int y, int z, int w) {
            Coordinate c = new Coordinate(x, y, z, w);
            if (!matrix.containsKey(c)) {
                if (tempNewCubes.containsKey(c)) {
                    return tempNewCubes.get(c);
                }
                return createCube(c, false);
            }
            return matrix.get(c);
        }

        public void cycle4D() {
            tempNewCubes.clear();
            matrix.values().forEach(hyperCube -> hyperCube.neighbors4D(this));
            tempNewCubes.forEach(matrix::put);
            tempNewCubes.clear();
            matrix.values().forEach(hyperCube -> hyperCube.preCycle4D(this));
            matrix.values().forEach(Cube::cycle);
        }
    }

    public static long part1(PocketDimension pocketDimension, int cycles) {
        for (int c = 1; c <= cycles; c++) {
            pocketDimension.cycle3D();
        }
        return pocketDimension.getNumActiveCubes();
    }

    public static long part2(PocketDimension pocketDimension, int cycles) {
        for (int c = 1; c <= cycles; c++) {
            pocketDimension.cycle4D();
        }
        return pocketDimension.getNumActiveCubes();
    }

    public static void main(String[] args) throws IOException {
        //final List<String> lines = readStringsBySeparator(new File("resources/sample_day_17.txt"), "\n");
        final List<String> lines = readStringsBySeparator(new File("resources/input_day_17.txt"), "\n");

        PocketDimension pocketDimension = new PocketDimension(lines);

        System.out.println("DAY 17 - Conway Cubes");
        System.out.println("Part 1: " + part1(pocketDimension, 6)); // 273
        System.out.println("==============================");

        pocketDimension = new PocketDimension(lines);
        System.out.println("Part 2: " + part2(pocketDimension, 6)); // 1504
    }
}