package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import static com.gubatron.aoc._2020.Utils.readStringList;

public class Day04 {
    static class Passport {
        HashMap<String, String> props = new HashMap<>();

        static Pattern hexColorPattern = Pattern.compile("#[0-9a-f]{6}");
        static Pattern abbrevColorPattern = Pattern.compile("[a-z]{3}");
        static Pattern heightPattern = Pattern.compile("\\d{2,3}[incm]{2}");
        static Pattern yearPattern = Pattern.compile("\\d{4}");
        static Pattern pidPattern = Pattern.compile("\\d{9}");

        Passport(List<String> lines) {
            lines.forEach(line -> {
                String[] raw_props = line.split(" ");
                Arrays.stream(raw_props).forEach(raw_prop -> {
                    String[] prop_arr = raw_prop.split(":");
                    props.put(prop_arr[0].toLowerCase(), prop_arr[1]);
                });
            });
        }

        void print() {
            System.out.println("Props: " + props.keySet().size());
            props.keySet().stream().sorted().forEach(p -> System.out.println(p + ":" + props.get(p)));
            System.out.println("\n");
        }

        boolean isValidSimple() {
            //        byr (Birth Year)
            //        iyr (Issue Year)
            //        eyr (Expiration Year)
            //        hgt (Height)
            //        hcl (Hair Color)
            //        ecl (Eye Color)
            //        pid (Passport ID)
            //        cid (Country ID) //OPTIONAL
            return props.containsKey("byr") &&
                    props.containsKey("iyr") &&
                    props.containsKey("eyr") &&
                    props.containsKey("hgt") &&
                    props.containsKey("hcl") &&
                    props.containsKey("ecl") &&
                    props.containsKey("pid");
        }


        boolean isValid() {
            return areYearsValid() &&
                    isHeightValid() &&
                    isHairColorValid() &&
                    isEyeColorValid() &&
                    pidIsValid();
        }

        boolean areYearsValid() {
            if (!props.containsKey("byr") ||
                    !props.containsKey("iyr") ||
                    !props.containsKey("eyr")) {
                return false;
            }
            Matcher byrMatcher = yearPattern.matcher(props.get("byr"));
            Matcher iyrMatcher = yearPattern.matcher(props.get("iyr"));
            Matcher eyrMatcher = yearPattern.matcher(props.get("eyr"));

            if (!(byrMatcher.matches() && iyrMatcher.matches() && eyrMatcher.matches())) {
                return false;
            }
            int birthYear = Integer.parseInt(props.get("byr"));
            boolean validBirthYear = 1920 <= birthYear && birthYear <= 2002;

            int issuanceYear = Integer.parseInt(props.get("iyr"));
            boolean validIssuanceYear = 2010 <= issuanceYear && issuanceYear <= 2020;

            int expYear = Integer.parseInt(props.get("eyr"));
            boolean validExpYear = 2020 <= expYear && expYear <= 2030;

            return validBirthYear && validIssuanceYear && validExpYear;
        }

        boolean isHeightValid() {
            if (!props.containsKey("hgt")) {
                return false;
            }
            String heightStr = props.get("hgt");
            Matcher heightMatcher = heightPattern.matcher(heightStr);
            if (!heightMatcher.matches()) {
                return false;
            }

            if (heightStr.contains("cm")) {
                int cms = Integer.parseInt(heightStr.replace("cm", ""));
                return 150 <= cms && cms <= 193;
            }

            if (heightStr.contains("in")) {
                int cms = Integer.parseInt(heightStr.replace("in", ""));
                return 59 <= cms && cms <= 76;
            }
            return false;
        }

        boolean pidIsValid() {
            if (!props.containsKey("pid")) return false;
            Matcher pidMatcher = pidPattern.matcher(props.get("pid"));
            return pidMatcher.matches();
        }

        boolean isHairColorValid() {
            if (!props.containsKey("hcl")) {
                return false;
            }
            Matcher hexMatcher = hexColorPattern.matcher(props.get("hcl"));
            return hexMatcher.matches();
        }

        boolean isEyeColorValid() {
            if (!props.containsKey("ecl")) {
                return false;
            }
            Matcher abbrevMatcher = abbrevColorPattern.matcher(props.get("ecl"));
            if (!abbrevMatcher.matches()) {
                return false;
            }
            String[] validColors = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth" };
            return Arrays.stream(validColors).anyMatch(s -> s.equals(props.get("ecl")));
        }
    }

    public static List<Passport> getPassports(List<String> lines) {
        final List<String> passport_accumulator = new ArrayList<>();
        List<Passport> passports = new ArrayList<>();
        lines.forEach(l -> {
            if (l.isEmpty()) {
                passports.add(new Passport(passport_accumulator));
                passport_accumulator.clear();
            } else {
                passport_accumulator.add(l);
            }
        });
        passports.add(new Passport(passport_accumulator)); // add the last one!
        return passports;
    }

    public static long part1(List<Passport> passports) {
        return passports.stream().map(passport -> passport.isValidSimple() ? 1 : 0).reduce(0, Integer::sum);
    }

    public static long part2(List<Passport> passports) {
        return passports.stream().map(passport -> passport.isValid() ? 1 : 0).reduce(0, Integer::sum);
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringList(new File("resources/sample_day_04.txt"));
        List<String> lines = readStringList(new File("resources/input_day_04.txt"));
        List<Passport> passports = getPassports(lines);
        System.out.println("DAY 04");
        System.out.println("Part 1: " + part1(passports)); // 242
        System.out.println("==============================\n");
        System.out.println("Part 2: " + part2(passports));
        System.out.println("==============================\n");
        System.out.println("Total Passports: " + passports.size()); //186
    }
}
