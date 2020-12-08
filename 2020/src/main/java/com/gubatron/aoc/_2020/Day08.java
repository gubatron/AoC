package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day08 {

    enum Code {
        ACC,
        JMP,
        NOP
    }

    static class Instruction {
        Code code;
        int argument;
        int timesExecuted = 0;
    }


    static int ACCUMULATOR = 0;
    static int PC = 0;

    private static List<Instruction> loadInstructions(List<String> lines) {
        return lines.stream().map(line -> {
            String cleanLine = line.replace("+", "");
            String[] s = cleanLine.split(" ");
            Instruction inst = new Instruction();
            switch (s[0]) {
                case "jmp" -> inst.code = Code.JMP;
                case "acc" -> inst.code = Code.ACC;
                case "nop" -> inst.code = Code.NOP;
            }
            inst.argument = Integer.parseInt(s[1]);
            return inst;
        }).collect(Collectors.toList());
    }

    private static int runProgram(final List<Instruction> instructions) {
        ACCUMULATOR = 0;
        PC = 0;
        while (PC < instructions.size()) {
            Instruction instruction = instructions.get(PC);
            if (instruction.timesExecuted == 1) {
                break;
            }
            instruction.timesExecuted++;
            if (instruction.code == Code.NOP) {
                PC++;
            } else if (instruction.code == Code.ACC) {
                PC++;
                ACCUMULATOR += instruction.argument;
                instruction.timesExecuted++;
            } else if (instruction.code == Code.JMP) {
                PC = PC + instruction.argument;
            }
        }
        return ACCUMULATOR;
    }

    public static long part1(List<Instruction> program) {
        return runProgram(program);
    }

    public static long part2(List<String> lines) {
        List<Instruction> program = loadInstructions(lines);
        List<Integer> candidateIndices = new ArrayList<>();
        int i = 0;
        for (Instruction instruction : program) {
            if (instruction.code == Code.JMP ||
                    instruction.code == Code.NOP) {
                candidateIndices.add(i);
            }
            i++;
        }

        for (Integer candidateInstructionIndex : candidateIndices) {
            PC = 0;
            ACCUMULATOR = 0;
            List<Instruction> testProgram = loadInstructions(lines);
            Instruction changeMe = testProgram.get(candidateInstructionIndex);
            if (changeMe.code == Code.JMP) {
                changeMe.code = Code.NOP;
            } else if (changeMe.code == Code.NOP) {
                changeMe.code = Code.JMP;
            } else {
                throw new RuntimeException("Check your logic!");
            }
            runProgram(testProgram);
            if (PC == testProgram.size()) {
                return ACCUMULATOR;
            }
        }
        return -1;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_08.txt"),"\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_08.txt"), "\n");
        List<Instruction> program = loadInstructions(lines);
        System.out.println("DAY 08 - Handheld Halting");
        System.out.println("Part 1: " + part1(program)); //1331
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(lines)); // 1121
    }
}
