package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.List;
import java.util.Stack;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day18 {

    static class Group {
        Stack<Long> operands = new Stack<>();
        Stack<Character> operators = new Stack<>();
        boolean readyToOperate = false;
    }

    static class Calculator {
        Stack<Group> stack = new Stack<>();
        Group currentGroup = new Group();

        Calculator(String exp) {
            exp.chars().filter(c -> c != ' ').forEach(c -> {
                final char C = (char) c;
                if (C == '(') {
                    currentGroup.readyToOperate = false;
                    stack.push(currentGroup);
                    currentGroup = new Group();
                } else if (C == ')') {
                    long b = currentGroup.operands.pop();
                    currentGroup = stack.pop();
                    currentGroup.operands.push(b);
                    currentGroup.readyToOperate = !currentGroup.operators.empty() && currentGroup.operands.size() > 1;
                    if (currentGroup.readyToOperate) {
                        operate();
                    }
                } else if (C == '+' || C == '-' || C == '*') {
                    currentGroup.readyToOperate = true;
                    currentGroup.operators.push(C);
                } else {
                    long b = Integer.parseInt(String.valueOf(C));
                    currentGroup.operands.push(b);
                    operate();
                }
            });
        }

        void operate() {
            if (currentGroup.readyToOperate) {
                long b = currentGroup.operands.pop();
                char lastOp = currentGroup.operators.pop();
                long a = currentGroup.operands.pop();
                if (lastOp == '+') {
                    currentGroup.operands.push(a + b);
                } else if (lastOp == '*') {
                    currentGroup.operands.push(a * b);
                }
                currentGroup.readyToOperate = false;
            }
        }

        public long answer() {
            return currentGroup.operands.pop();
        }
    }

    static class AdvancedCalculator {
        Stack<Group> stack = new Stack<>();
        Group currentGroup = new Group();

        AdvancedCalculator(String exp) {
            exp.chars().filter(c -> c != ' ').forEach(c -> {
                final char C = (char) c;
                if (C == '(') {
                    currentGroup.readyToOperate = false;
                    stack.push(currentGroup);
                    currentGroup = new Group();
                } else if (C == ')') {
                    currentGroup.readyToOperate = true;
                    while (currentGroup.operands.size() > 1 && !currentGroup.operators.isEmpty()) {
                        operate();
                        currentGroup.readyToOperate = true;
                    }
                    long result = currentGroup.operands.pop();
                    currentGroup = stack.pop();
                    currentGroup.operands.push(result);
                    currentGroup.readyToOperate = readyForAddition() || readyForMultiplication();
                    operate();
                } else if (C == '+') {
                    currentGroup.operators.push(C);
                    currentGroup.readyToOperate = readyForAddition();
                } else if (C == '*') {
                    currentGroup.operators.push(C);
                    currentGroup.readyToOperate = readyForMultiplication();
                } else {
                    long b = Integer.parseInt(String.valueOf(C));
                    currentGroup.operands.push(b);
                    currentGroup.readyToOperate = readyForAddition() || readyForMultiplication();
                    if (currentGroup.readyToOperate) {
                        operate();
                    }
                }
            });
            while (currentGroup.operands.size() > 1 && !currentGroup.operators.isEmpty()) {
                operate();
                currentGroup.readyToOperate = true;
            }
        }

        boolean plusInOperatorsStack() {
            return currentGroup.operators.stream().anyMatch(c -> c == '+');
        }

        boolean readyForAddition() {
            return !currentGroup.operators.isEmpty() &&
                    (currentGroup.operands.size() == currentGroup.operators.size() + 1) &&
                    currentGroup.operators.peek() == '+';
        }

        boolean readyForMultiplication() {
            return !currentGroup.operators.isEmpty() && plusInOperatorsStack();
        }

        void operate() {
            if (currentGroup.readyToOperate) {
                long b = currentGroup.operands.pop();
                char lastOp = currentGroup.operators.pop();
                long a = currentGroup.operands.pop();
                if (lastOp == '+') {
                    currentGroup.operands.push(a + b);
                } else if (lastOp == '*') {
                    currentGroup.operands.push(a * b);
                }
                currentGroup.readyToOperate = false;
            }
        }

        public long answer() {
            return currentGroup.operands.pop();
        }

    }

    public static long part1(List<String> lines) {
        long sum = 0;
        for (String exp : lines) {
            Calculator calculator = new Calculator(exp);
            sum += calculator.answer();
        }
        return sum;
    }

    public static long part2(List<String> lines) {
        long sum = 0;
        for (String exp : lines) {
            AdvancedCalculator calculator = new AdvancedCalculator(exp);
            sum += calculator.answer();
        }
        return sum;
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_18.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_18.txt"), "\n");
        System.out.println("DAY 18 - Operation Order");
        System.out.println("Part 1: " + part1(lines)); // 6640667297513
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(lines)); // 451589894841552
    }
}
