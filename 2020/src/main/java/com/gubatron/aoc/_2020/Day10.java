package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.math.BigInteger;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day10 {
    public static long part1(List<Long> joltages) {
        long dev_high_joltage = joltages.get(joltages.size() - 1) + 3;
        HashMap<Long, Long> diffs = new HashMap<>();

        joltages.add(0, 0L);
        joltages.add(dev_high_joltage);


        for (int i = 0; i < joltages.size() - 1; i++) {
            long d = joltages.get(i + 1) - joltages.get(i);
            if (diffs.containsKey(d)) {
                diffs.put(d, 1 + diffs.get(d));
            } else {
                diffs.put(d, 1L);
            }
        }

        return diffs.get(1L) * diffs.get(3L);
    }

    public static BigInteger toBigInt(List<Long> js) {
        StringBuilder sb = new StringBuilder();
        long last = js.get(js.size() - 1);
        for (Long l : js) {
            sb.append(l);
        }
        return new BigInteger(sb.toString());
    }

    public static long getNumValidSubchains(List<Long> inputs, HashMap<BigInteger, Long> cache) {
        if (inputs.size() == 1) {
            // was able to find a path that reduced the chain to one element
            return 1;
        }

        long validChainsCount = 0;
        int index = 1;
        long current = inputs.get(0);
        while (inputs.size() > index && inputs.get(index) - current < 4) {
            List<Long> subList = inputs.subList(index, inputs.size());
            BigInteger subListKey = toBigInt(subList);
            if (cache.containsKey(subListKey)) {
                validChainsCount += cache.get(subListKey);
            } else {
                long subArrangements = getNumValidSubchains(subList, cache);
                cache.put(subListKey, subArrangements);
                validChainsCount += subArrangements;
            }
            index++;
        }
        return validChainsCount;
    }

    public static long part2(List<Long> chain) {
        return getNumValidSubchains(chain, new HashMap<>());
    }


    public static void main(String[] args) throws IOException, InterruptedException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_10.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_10.txt"), "\n");
        List<Long> joltages = lines.stream().map(Long::parseLong).collect(Collectors.toList());
        List<Long> sortedJoltages = joltages.stream().sorted().collect(Collectors.toList());

        System.out.println("DAY 10 - Adapter Array");

        System.out.println("Part 1: " + part1(sortedJoltages)); // 2368
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(sortedJoltages)); // 1727094849536 (in 92ms)
    }
}
