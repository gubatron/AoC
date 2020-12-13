package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day13 {
    public static int part1(int timestamp, List<Integer> busIds) {
        int bestTimeDiff = Integer.MAX_VALUE;
        int candidateBusId = -1;
        for (Integer busId : busIds) {
            int timeDiff = busId - (timestamp % busId);
            if (timeDiff < bestTimeDiff) {
                candidateBusId = busId;
                bestTimeDiff = timeDiff;
            }
        }
        return candidateBusId * bestTimeDiff;
    }

    static class Bus {
        int id;
        int offset;
        BigInteger idBigInt;
        BigInteger offsetBigInt;

        public Bus(Integer busID, int i) {
            id = busID;
            offset = i;
            idBigInt = new BigInteger(String.valueOf(id));
            offsetBigInt = new BigInteger(String.valueOf(offset));
        }

        // meant to be used for part 2, but couldn't figure out how to increment test T fast enough
        // however, this served to show me what the system of equations would look like eventually
        // had to do this analytically
        boolean isTValidTForMe(final BigInteger t) {
            try {
                // (T+MyOffset) % myID == 0
                return t.add(offsetBigInt).mod(idBigInt).compareTo(BigInteger.ZERO) == 0;
            } catch (Throwable ignore) {
                return false;
            }
        }
    }

    // Did this when I was trying to validate the sample inputs for different attempts of t
    // I just could never figure out how to increment t the right way without burning my CPU
    static boolean isTValidForAllBuses(final BigInteger t, List<Bus> buses) {
        for (Bus bus : buses) {
            if (!bus.isTValidTForMe(t)) {
                return false;
            }
        }
        return true;
    }

    public static String part2(List<Integer> busIds) {
        int offset = 0;
        System.out.println("Find t such that:");
        for (Integer busID : busIds) {
            if (busID == -1) { // skip the Xs
                offset++; //but keep track of the offsets
                continue;
            }
            System.out.print("(t + " + offset++ + ") mod " + busID + " = 0,\n");
        }

        return "\nCopy and paste that in Wolfram Alpha!\n\n402251700208309 + 748958695773119 n\n\nFor n=0 -> Ans=402251700208309";
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_13.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_13.txt"),"\n");

        int timestamp = Integer.parseInt(lines.get(0));
        List<Integer> busIds = Arrays.stream(lines.get(1).split(",")).filter(s -> !s.equals("x")).map(Integer::parseInt).collect(Collectors.toList());

        System.out.println("DAY 13 - Shuttle Search");
        System.out.println("Part 1: " + part1(timestamp, busIds)); // 2045 (sample 295)
        System.out.println("==============================");

        List<Integer> busIds2 = Arrays.stream(lines.get(1).split(",")).map(s -> {
            if (s.equals("x")) {
                return -1; // mark Xs as -1 bus line ids
            } else return Integer.parseInt(s);
        }).collect(Collectors.toList());

        System.out.print("Part 2: ");
        System.out.println(part2(busIds2)); // 402251700208309
    }
}
