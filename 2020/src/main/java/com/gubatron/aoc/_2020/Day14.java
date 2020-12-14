package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day14 {
    public static class Bits {
        public static BitSet convert(long value) {
            BitSet bits = new BitSet();
            int index = 0;
            while (value != 0L) {
                if (value % 2L != 0) {
                    bits.set(index);
                }
                ++index;
                value = value >>> 1;
            }
            return bits;
        }

        public static long convert(BitSet bits) {
            long value = 0L;
            for (int i = 0; i < bits.length(); ++i) {
                value += bits.get(i) ? (1L << i) : 0L;
            }
            return value;
        }
    }

    static Pattern memPattern = Pattern.compile("mem\\[(\\d+)] = (\\d+)");

    static class Instruction {
        // part 1
        long address;
        int originalValue;
        BitSet bitsetValue;
        // part 2
        BitSet addressBitset;
        List<Integer> floatAddressMaskedBitOffsets;

        Instruction(String line) {
            Matcher matcher = memPattern.matcher(line);
            if (matcher.find()) {
                address = Integer.parseInt(matcher.group(1));
                addressBitset = Bits.convert(address);
                originalValue = Integer.parseInt(matcher.group(2));
                bitsetValue = Bits.convert(originalValue);
            }
        }

        // Applies bitmask to value
        void applyMask(char[] mask) {
            for (int i = 35; i >= 0; i--) {
                int j = 35 - i;
                char bitMask = mask[i];
                if (bitMask == 'X') continue;
                bitsetValue.set(j, bitMask == '1');
            }
        }

        // Applies bitmask to address, keeps track of what bits we have to float for possible addresses
        void applyMaskV2(char[] mask) {
            floatAddressMaskedBitOffsets = new ArrayList<>();
            for (int i = 35; i >= 0; i--) {
                int j = 35 - i;
                char bitMask = mask[i];
                if (bitMask == 'X') {
                    floatAddressMaskedBitOffsets.add(j);
                } else if (bitMask == '1') {
                    addressBitset.set(j, true);
                }
            }
        }

        long getValueFromBitset() {
            return Bits.convert(bitsetValue);
        }

        List<Long> getFloatingMemoryAddresses() {
            List<Long> addresses = new ArrayList<>();
            int possibleAddresses = (int) Math.pow(2, floatAddressMaskedBitOffsets.size());
            for (int i = 0; i < possibleAddresses; i++) {
                // Create all the possible numbers from 0 to N
                // Convert to binary and insert into 0 padded char[]
                // Then flip each one of those bits in the given floating memory masked bit positions
                StringBuilder iAsBinary = new StringBuilder(Integer.toBinaryString(i));
                while (iAsBinary.length() < floatAddressMaskedBitOffsets.size()) {
                    iAsBinary.insert(0, "0");
                }
                char[] bitChars = iAsBinary.toString().toCharArray();
                BitSet address = (BitSet) addressBitset.clone();
                for (int j = 0; j < bitChars.length; j++) {
                    int bitPosition = floatAddressMaskedBitOffsets.get(j);
                    address.set(bitPosition, bitChars[j] == '1');
                }
                long addressAsLong = Bits.convert(address);
                addresses.add(addressAsLong);
            }
            return addresses;
        }
    }

    enum ParseMode {
        VALUE_MASK_MODE,
        ADDRESS_MASK_MODE
    }

    static class Program {
        public char[] mask;
        List<Instruction> instructions = new ArrayList<>();
        HashMap<Long, Long> memory = new HashMap<>();

        Program() {
        }

        void parseValueInstruction(String instruction) {
            parseInstruction(instruction, ParseMode.VALUE_MASK_MODE);
        }

        void parseAddressInstruction(String instruction) {
            parseInstruction(instruction, ParseMode.ADDRESS_MASK_MODE);
        }

        private void parseInstruction(String instruction, ParseMode parseMode) {
            String[] inst = instruction.split(" = ");
            if (inst[0].startsWith("mask")) {
                updateMask(inst[1]);
            } else if (inst[0].startsWith("mem")) {
                if (parseMode == ParseMode.VALUE_MASK_MODE) {
                    runMemInstruction(instruction);
                } else if (parseMode == ParseMode.ADDRESS_MASK_MODE) {
                    runAddressInstruction(instruction);
                }
            }
        }

        void updateMask(String maskString) {
            mask = maskString.toCharArray();
        }

        void runMemInstruction(String line) {
            Instruction instruction = new Instruction(line);
            instruction.applyMask(mask);
            memory.put(instruction.address, instruction.getValueFromBitset());
        }

        void runAddressInstruction(String line) {
            Instruction instruction = new Instruction(line);
            instruction.applyMaskV2(mask);
            instruction.getFloatingMemoryAddresses().forEach(
                    address -> memory.put(address, instruction.getValueFromBitset()));
        }

        long sumMemoryValues() {
            return memory.keySet().stream().map(address -> memory.get(address)).reduce(Long::sum).orElseThrow();
        }
    }

    static long part1(List<String> lines) {
        Program program = new Program();
        lines.forEach(program::parseValueInstruction);
        return program.sumMemoryValues();
    }

    static long part2(List<String> lines) {
        Program program = new Program();
        lines.forEach(program::parseAddressInstruction);
        return program.sumMemoryValues();
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_14.txt"), "\n");
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_14_1.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_14.txt"), "\n");
        System.out.println("DAY 14 - Docking Data");
        System.out.println("Part 1: " + part1(lines)); // 12135523360904 (Sample 1: 165)
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(lines)); // 2741969047858 (Sample 2: 208)
    }
}
