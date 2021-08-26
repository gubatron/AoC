package foobar.level2;

import java.util.Arrays;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Solution {

    private static class Version implements Comparable<Version> {

        static Pattern pattern_major = Pattern.compile("^(\\d+)$");
        static Pattern pattern_major_minor = Pattern.compile("^(\\d+)\\.(\\d+)$");
        static Pattern pattern_major_minor_revision = Pattern.compile("^(\\d+)\\.(\\d+)\\.(\\d+)$");

        // "1"
        int major;
        int minor;
        int rev;

        Version(String v) {
            Matcher matcher_pattern_major_minor_revision = pattern_major_minor_revision.matcher(v);
            if (matcher_pattern_major_minor_revision.matches()) {
                major = Integer.parseInt(matcher_pattern_major_minor_revision.group(1));
                minor = Integer.parseInt(matcher_pattern_major_minor_revision.group(2));
                rev = Integer.parseInt(matcher_pattern_major_minor_revision.group(3));
                return;
            }

            Matcher matcher_pattern_major_minor = pattern_major_minor.matcher(v);
            if (matcher_pattern_major_minor.matches()) {
                major = Integer.parseInt(matcher_pattern_major_minor.group(1));
                minor = Integer.parseInt(matcher_pattern_major_minor.group(2));
                rev = -1;
                return;
            }

            Matcher matcher_pattern_major = pattern_major.matcher(v);
            if (matcher_pattern_major.matches()) {
                major = Integer.parseInt(matcher_pattern_major.group(1));
                minor = -1;
                rev = -1;
            }
        }

        @Override
        public String toString() {
            if (minor == -1 && rev == -1) {
                return String.valueOf(major);
            }
            if (rev == -1) {
                return String.format("%d.%d", major, minor);
            }
            return String.format("%d.%d.%d", major, minor, rev);
        }

        @Override
        public int compareTo(Version v2) {
            if (major == v2.major) {
                if (minor == v2.minor) {
                    return Integer.compare(rev, v2.rev);
                } else {
                    return Integer.compare(minor, v2.minor);
                }
            }
            return Integer.compare(major, v2.major);
        }
    }

    public static String[] solution(String[] l) {
        // map back to Stream<String>
        return (String[]) Arrays.stream(l).sequential().
                map(Version::new). // map to Stream<Version>
                sorted(). // sort using the Version comparator
                map(Version::toString).collect(Collectors.toList()).toArray(new String[]{}); // convert to String array
    }

    public static void tests() {
        System.out.println("foobar.level2.elevator-maintenance");
        Arrays.stream(solution(new String[]{"1.11", "2.0.0", "1.2", "2", "0.1", "1.2.1", "1.1.1", "2.0"})).forEach(v -> {
            System.out.print(v + ",");
        });
        System.out.println();
        Arrays.stream(solution(new String[]{"1.1.2", "1.0", "1.3.3", "1.0.12", "1.0.2"})).forEach(v -> {
            System.out.print(v + ",");
        });
    }
}
