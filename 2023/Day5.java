import java.io.IOException;
import java.math.BigInteger;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;

public class Day5 {

    public static class SeedMap {
        private String name;
        private List<SeedRange> ranges;

        public SeedMap(String name, List<SeedRange> ranges) {
            this.name = name;
            this.ranges = ranges;
        }

        public BigInteger getDest(BigInteger seed) {
            for (SeedRange r : ranges) {
                if (r.inSourceRange(seed)) {
                    return r.getDest(seed);
                }
            }
            return seed;
        }
    }

    public static class SeedRange {
        private final BigInteger dest;
        private final BigInteger source;
        private final BigInteger length;

        public SeedRange(BigInteger dest, BigInteger source, BigInteger length) {
            this.dest = dest;
            this.source = source;
            this.length = length;
        }

        public boolean inSourceRange(BigInteger seed) {
            return seed.compareTo(source) >= 0 && seed.compareTo(source.add(length)) <= 0;
        }

        public BigInteger getDest(BigInteger seed) {
            if (this.inSourceRange(seed)) {
                return seed.add(dest.subtract(source));
            } else {
                return seed;
            }
        }
    }

    public static List<BigInteger> parseSeeds(List<String> lines) {
        String[] parts = lines.get(0).split(": ")[1].split(" ");
        List<BigInteger> seeds = new ArrayList<>();
        for (String part : parts) {
            seeds.add(new BigInteger(part));
        }
        return seeds;
    }

    public static List<BigInteger[]> parseSeeds2(List<String> lines) {
        List<BigInteger> seedNumbers = parseSeeds(lines);
        List<BigInteger[]> seedRanges = new ArrayList<>();
        for (int i = 0; i < seedNumbers.size(); i += 2) {
            seedRanges.add(new BigInteger[]{seedNumbers.get(i), seedNumbers.get(i + 1)});
        }
        Collections.reverse(seedRanges);
        return mergeRanges(seedRanges);
    }

    public static List<BigInteger[]> mergeRanges(List<BigInteger[]> ranges) {
        ranges.sort(Comparator.comparing(a -> a[0]));
        List<BigInteger[]> merged = new ArrayList<>();
        merged.add(ranges.get(0));

        for (int i = 1; i < ranges.size(); i++) {
            BigInteger[] current = ranges.get(i);
            BigInteger[] last = merged.get(merged.size() - 1);

            BigInteger lastEnd = last[0].add(last[1]).subtract(BigInteger.ONE);
            BigInteger currentEnd = current[0].add(current[1]).subtract(BigInteger.ONE);

            if (current[0].compareTo(lastEnd) <= 0) {
                last[1] = lastEnd.max(currentEnd).subtract(last[0]).add(BigInteger.ONE);
            } else {
                merged.add(new BigInteger[]{current[0], current[1]});
            }
        }
        return merged;
    }

    public static List<SeedMap> parseSeedMaps(List<String> lines) {
        List<SeedMap> seedMaps = new ArrayList<>();
        int i = 2;
        while (i < lines.size()) {
            String line = lines.get(i);
            SeedMap currentSeedMap = new SeedMap("", new ArrayList<>());
            if (line.contains("-to-")) {
                currentSeedMap = new SeedMap(line, new ArrayList<>());
                i++;
                while (i < lines.size() && !lines.get(i).isEmpty()) {
                    line = lines.get(i);
                    String[] seedRangeData = line.split(" ");
                    SeedRange currentSeedRange = new SeedRange(
                            new BigInteger(seedRangeData[0]),
                            new BigInteger(seedRangeData[1]),
                            new BigInteger(seedRangeData[2])
                    );
                    currentSeedMap.ranges.add(currentSeedRange);
                    i++;
                }
                seedMaps.add(currentSeedMap);
            }
            i++;
        }
        return seedMaps;
    }

    public static BigInteger feedSeedThroughSeedMapPipeline(BigInteger seed, List<SeedMap> seedMaps) {
        for (SeedMap sm : seedMaps) {
            seed = sm.getDest(seed);
        }
        return seed;
    }

    public static BigInteger findLowestLocation(List<BigInteger> seeds, List<SeedMap> seedMaps) {
        BigInteger lowestLocation = new BigInteger("999999999999999999");
        for (BigInteger seed : seeds) {
            lowestLocation = lowestLocation.min(feedSeedThroughSeedMapPipeline(seed, seedMaps));
        }
        return lowestLocation;
    }

    public static BigInteger findLowestLocation2(List<BigInteger[]> mergedRanges, List<SeedMap> seedMaps) throws InterruptedException {
        ExecutorService executor = Executors.newFixedThreadPool(8);
        CountDownLatch latch = new CountDownLatch(mergedRanges.size());
        AtomicReference<BigInteger> lowestLocation = new AtomicReference<>(new BigInteger("999999999999999999"));

        for (BigInteger[] range : mergedRanges) {
            executor.execute(() -> {
                BigInteger rangeStart = range[0];
                BigInteger rangeEnd = rangeStart.add(range[1]);
                System.out.println("findLowestLocation2 -> Range: " + rangeStart + " - " + rangeEnd);
                BigInteger localLowest = new BigInteger("999999999999999999");
                for (BigInteger seed = rangeStart; seed.compareTo(rangeEnd) < 0; seed = seed.add(BigInteger.ONE)) {
                    localLowest = localLowest.min(feedSeedThroughSeedMapPipeline(seed, seedMaps));
                }
                synchronized (lowestLocation) {
                    lowestLocation.set(lowestLocation.get().min(localLowest));
                }
                latch.countDown();
            });
        }

        latch.await();
        executor.shutdown();

        return lowestLocation.get();
    }

    public static void part1(String dataFile) throws IOException {
        List<String> lines = Files.readAllLines(Paths.get(dataFile));
        List<BigInteger> seeds = parseSeeds(lines);
        List<SeedMap> seedMaps = parseSeedMaps(lines);
        System.out.println("Part 1: Lowest location = " + findLowestLocation(seeds, seedMaps));
    }

    public static void part2(String dataFile) throws IOException, InterruptedException {
        List<String> lines = Files.readAllLines(Paths.get(dataFile));
        List<BigInteger[]> mergedRanges = parseSeeds2(lines);
        List<SeedMap> seedMaps = parseSeedMaps(lines);
        System.out.println("Part 2: Lowest location = " + findLowestLocation2(mergedRanges, seedMaps));
    }

    // main method stays the same
    public static void main(String[] args) throws IOException, InterruptedException {
        String dataFile = "5.txt";
        part1(dataFile);
        part2(dataFile);
    }
}
