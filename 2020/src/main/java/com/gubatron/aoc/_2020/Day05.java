package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringList;

public class Day05 {
    static class Seat {
        final String ticket;
        int row;
        int col;
        int row_start = 0;
        int row_end = 127;
        int col_start = 0;
        int col_end = 7;

        Seat(int row_p, int col_p) {
            ticket = row + ":" + col;
            row = row_p;
            col = col_p;
        }

        Seat(String ticket_p) {
            ticket = ticket_p;
            char[] t = ticket.toCharArray();

            // Row detect
            for (int i = 0; i < 7; i++) {
                int distance = (int) Math.floor((row_end - row_start) / 2) + 1;
                if (t[i] == 'F') {
                    row_end = row_end - distance;
                } else if (t[i] == 'B') {
                    row_start = row_start + distance;
                }
            }
            row = t[6] == 'F' ? Math.min(row_start, row_end) : Math.max(row_start, row_end);

            // Col detect
            for (int i = 7; i < 10; i++) {
                int distance = (int) Math.floor((col_end - col_start) / 2) + 1;
                if (t[i] == 'L') { // LOWER HALF
                    col_end = col_end - distance;
                } else if (t[i] == 'R') { // UPPER HALF
                    col_start = col_start + distance;
                }
            }
            col = t[9] == 'L' ? Math.min(col_start, col_end) : Math.max(col_start, col_end);
        }

        public int getID() {
            return row * 8 + col;
        }

        @Override
        public boolean equals(Object o) {
            Seat other = (Seat) o;
            return getID() == other.getID();
        }

        @Override
        public int hashCode() {
            return getID();
        }
    }

    public static List<Seat> getSeats(List<String> lines) {
        return lines.stream().map(Seat::new).collect(Collectors.toList());
    }

    private static long highest_seat_id_part_01;

    public static long part1(List<Seat> seats) {
        seats.forEach(s -> {
            if (s.getID() > highest_seat_id_part_01) {
                highest_seat_id_part_01 = s.getID();
            }
        });
        return highest_seat_id_part_01;
    }

    static class SeatTracker {
        Seat lastSeat = null;
        int myTicketID = -1;
    }

    public static long part2(List<Seat> seats) {
        // Sort the list, and keep track of seats, when the difference in IDs is 2, we have our ticket.
        SeatTracker TRACKER = new SeatTracker();
        List<Seat> sortedSeats = seats.stream().sorted(Comparator.comparingInt(Seat::getID)).collect(Collectors.toList());
        Seat first = sortedSeats.get(0);
        Seat last = sortedSeats.get(sortedSeats.size() - 1);
        sortedSeats.stream().
                takeWhile(s -> TRACKER.myTicketID == -1). // Keep looping until we find it
                forEach(seat -> {
            if (TRACKER.lastSeat != null && seat.getID() - TRACKER.lastSeat.getID() == 2) {
                TRACKER.myTicketID = seat.getID() - 1; // makes take while stop
            } else {
                TRACKER.lastSeat = seat;
            }
        });

        return TRACKER.myTicketID;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringList(new File("resources/sample_day_05.txt"));
        List<String> lines = readStringList(new File("resources/input_day_05.txt"));
        List<Seat> seats = getSeats(lines);
        System.out.println("DAY 05");
        System.out.println("Part 1: " + part1(seats)); // 904
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(seats)); // 669
    }
}