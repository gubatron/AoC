package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.function.BinaryOperator;
import java.util.stream.Stream;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day11 {
    public enum State {
        EMPTY,
        OCCUPIED,
        FLOOR
    }

    public static class Cell {
        int x;
        int y;
        boolean willFlip;
        public State state;
        List<State> adjacent;

        @Override
        public String toString() {
            return "Cell{" +
                    "x=" + x +
                    ", y=" + y +
                    ", state=" + state +
                    ", adjacent=" + adjacent +
                    '}';
        }

        public Cell(char c, int x, int y) {
            if (c == 'L') {
                state = State.EMPTY;
            } else if (c == '#') {
                state = State.OCCUPIED;
            } else if (c == '.') {
                state = State.FLOOR;
            }
            this.x = x;
            this.y = y;
        }

        List<Cell> getAdjacent(final List<List<Cell>> layout) {
            List<Cell> result = new ArrayList<>();

            List<Cell> row_above = new ArrayList<>();
            if (y > 0) {
                row_above.addAll(layout.get(y - 1));
            }
            List<Cell> row_same = layout.get(y);
            List<Cell> row_below = new ArrayList<>();
            if ((y + 1) < layout.size()) {
                row_below.addAll(layout.get(y + 1));
            }

            //TOP LEFT
            if (x > 0 && y > 0) {
                result.add(row_above.get(x - 1));
            }
            // TOP
            if (y > 0) {
                result.add(row_above.get(x));
            }
            //TOP RIGHT
            if (y > 0 && x + 1 < row_same.size()) {
                result.add(row_above.get(x + 1));
            }
            //LEFT
            if (x > 0) {
                result.add(row_same.get(x - 1));
            }
            //RIGHT
            if (x + 1 < row_same.size()) {
                result.add(row_same.get(x + 1));
            }
            // BOTTOM LEFT
            if (x > 0 && (y + 1) < layout.size()) {
                result.add(row_below.get(x - 1));
            }
            // BOTTOM
            if (y + 1 < layout.size()) {
                result.add(row_below.get(x));
            }
            // BOTTOM RIGHT
            if (x + 1 < row_same.size() && y + 1 < layout.size()) {
                result.add(row_below.get(x + 1));
            }
            return result;
        }

        List<Cell> getVisibleSeats(final List<List<Cell>> layout) {
            List<Cell> visible = new ArrayList<>();

            //TOP LEFT
            if (x > 0 && y > 0) {
                int xx = x - 1;
                int yy = y - 1;
                while (xx >= 0 && yy >= 0 && layout.get(yy).get(xx).state == State.FLOOR) {
                    xx--;
                    yy--;
                }
                if (xx >= 0 && yy >= 0) {
                    visible.add(layout.get(yy).get(xx));
                }
            }

            //TOP
            if (y > 0) {
                int yy = y - 1;
                while (yy >= 0 && layout.get(yy).get(x).state == State.FLOOR) {
                    yy--;
                }
                if (yy >= 0) {
                    visible.add(layout.get(yy).get(x));
                }
            }

            //TOP RIGHT
            if (y > 0 && x + 1 < layout.get(0).size()) {
                int xx = x + 1;
                int yy = y - 1;
                while (yy >= 0 &&
                        xx <= layout.get(0).size() - 1 &&
                        layout.get(yy).get(xx).state == State.FLOOR) {
                    xx++;
                    yy--;
                }
                if (yy >= 0 && xx <= layout.get(0).size() - 1) {
                    visible.add(layout.get(yy).get(xx));
                }
            }

            //LEFT
            if (x > 0) {
                int xx = x - 1;
                while (xx >= 0 && layout.get(y).get(xx).state == State.FLOOR) {
                    xx--;
                }
                if (xx >= 0) {
                    visible.add(layout.get(y).get(xx));
                }
            }

            //RIGHT
            if (x + 1 < layout.get(0).size()) {
                int xx = x + 1;
                while (xx <= layout.get(0).size() - 1 && layout.get(y).get(xx).state == State.FLOOR) {
                    xx++;
                }
                if (xx <= layout.get(0).size() - 1) {
                    visible.add(layout.get(y).get(xx));
                }
            }

            //BOTTOM LEFT
            if (x > 0 && (y + 1) < layout.size()) {
                int xx = x - 1;
                int yy = y + 1;
                while (xx >= 0 &&
                        yy <= layout.size() - 1 &&
                        layout.get(yy).get(xx).state == State.FLOOR) {
                    xx--;
                    yy++;
                }
                if (xx >= 0 && yy <= layout.size() - 1) {
                    visible.add(layout.get(yy).get(xx));
                }
            }

            //BOTTOM
            if ((y + 1) < layout.size()) {
                int yy = y + 1;
                while (yy <= layout.size() - 1 && layout.get(yy).get(x).state == State.FLOOR) {
                    yy++;
                }
                if (yy <= layout.size() - 1) {
                    visible.add(layout.get(yy).get(x));
                }
            }

            //BOTTOM RIGHT
            if ((y + 1) < layout.size() &&
                    x + 1 < layout.get(0).size()) {
                int xx = x + 1;
                int yy = y + 1;
                while (yy < layout.size() &&
                        xx < layout.get(0).size() &&
                        layout.get(yy).get(xx).state == State.FLOOR) {
                    xx++;
                    yy++;
                }
                if (yy < layout.size() &&
                        xx < layout.get(0).size()) {
                    visible.add(layout.get(yy).get(xx));
                }
            }

            return visible;
        }

        long countOccupiedCellsAround(final List<List<Cell>> layout) {
            return getAdjacent(layout).stream().filter(cell -> cell.state == State.OCCUPIED).count();
        }

        long countVisibleOccupiedCellsAround(final List<List<Cell>> layout) {
            return getVisibleSeats(layout).stream().filter(cell -> cell.state == State.OCCUPIED).count();
        }

        void markForFlipping() {
            willFlip = true;
        }

        int flip() {
            if (!willFlip) return 0;
            if (state == State.EMPTY) {
                state = State.OCCUPIED;
            } else if (state == State.OCCUPIED) {
                state = State.EMPTY;
            }
            willFlip = false;
            return 1;
        }
    }

    static void fillRow(String line, int rowNum, List<Cell> row) {
        char[] chars = line.toCharArray();
        int x = 0;
        for (char c : chars) {
            row.add(new Cell(c, x++, rowNum));
        }
    }

    static List<List<Cell>> buildLayout(List<String> lines) {
        List<List<Cell>> layout = new ArrayList<>();
        int rowNum = 0;
        for (String l : lines) {
            List<Cell> row = new ArrayList<>();
            fillRow(l, rowNum++, row);
            layout.add(row);
        }
        return layout;
    }

    public static long countSeatsInState(State state, final List<List<Cell>> layout) {
        return layout.stream().map(row -> row.stream().filter(cell -> cell.state == state).count()).reduce(Long::sum).orElse(0L);
    }

    private static int flipCells(List<List<Cell>> layout) {
        return layout.stream().flatMap(row -> row.stream().map(Cell::flip)).reduce(Integer::sum).orElse(0);
    }

    public static long part1(final List<List<Cell>> layout) {
        do {
            layout.forEach(row -> row.forEach(cell -> {
                if ((cell.state == State.EMPTY && cell.countOccupiedCellsAround(layout) == 0) ||
                        (cell.state == State.OCCUPIED && cell.countOccupiedCellsAround(layout) >= 4)) {
                    // THEY ALL CHANGE AT ONCE, MARK THEM FOR FLIPPING BEFORE CHANGING LAYOUT STATE
                    cell.markForFlipping();
                }
            }));
            // THEN WE FLIP
        } while (flipCells(layout) != 0);
        return countSeatsInState(State.OCCUPIED, layout);
    }

    public static long part2(final List<List<Cell>> layout) {
        do {
            layout.forEach(row -> row.forEach(cell -> {
                if ((cell.state == State.EMPTY && cell.countVisibleOccupiedCellsAround(layout) == 0) ||
                        (cell.state == State.OCCUPIED && cell.countVisibleOccupiedCellsAround(layout) >= 5)) {
                    // THEY ALL CHANGE AT ONCE, MARK THEM FOR FLIPPING BEFORE CHANGING LAYOUT STATE
                    cell.markForFlipping();
                }
            }));
            // THEN WE FLIP
        } while (flipCells(layout) != 0);
        return countSeatsInState(State.OCCUPIED, layout);
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_11.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_11.txt"), "\n");
        System.out.println("DAY 11 - Seating System");
        List<List<Cell>> layout = buildLayout(lines);
        System.out.println("Part 1: " + part1(layout)); // 2194
        System.out.println("==============================");
        layout = buildLayout(lines);
        System.out.println("Part 2: " + part2(layout)); //1944
    }
}