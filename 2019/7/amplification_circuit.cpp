// Author: @gubatron December 21, 2019
#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

typedef int OpCode;
const OpCode ADD = 1;
const OpCode MULTIPLY = 2;
const OpCode INPUT = 3;
const OpCode OUTPUT = 4;
const OpCode JUMP_IF_TRUE = 5;
const OpCode JUMP_IF_FALSE = 6;
const OpCode LESS_THAN = 7;
const OpCode EQUALS = 8;
const OpCode END = 99;

typedef int ParameterMode;
const ParameterMode POSITION = 0;
const ParameterMode IMMEDIATE = 1;

typedef struct IntCode {
  int modes_n_opcode;
  OpCode op_code;
  ParameterMode mode_operand_a;
  ParameterMode mode_operand_b;
  ParameterMode mode_operand_c;
  int operand_a;
  int operand_b;
  int operand_c;
  int next_offset;
} IntCode;

typedef struct VM {
  int pc;
  std::vector<int> tape;
  std::deque<int> inputs;
  int output;
  VM(std::vector<int> program) : pc(0), tape(program) {}
} VM;

std::array<int, 5> to_array(int n);
std::vector<int> read_program();
IntCode read_instruction(VM &tape);
void run_instruction(VM &vm,
                     IntCode const &instruction);
void run_program(VM &vm, int pc);
std::string asSequence(int input);
void part1();
void part2();
int amplifier(VM &vm, int phase_setting, int input_signal);
int amplify_signal(int sequence, std::vector<int> const tape);
std::vector<int> phase_sequence_generator();

int main() {
  part1();
  return 0;
}

void part1() {
  std::vector<int> tape = read_program();
  std::vector<int> sequences = phase_sequence_generator();
  int maxInput = 0, maxOutput = 0;
  for (auto it = sequences.begin(); it != sequences.end(); it++) {    
    int output;
    if ((output = amplify_signal(*it, tape)) > maxOutput) {
      maxInput = *it;
      maxOutput = output;
    }
  }
   std::cout << "Part 1:  Max Thruster Signal = " << maxOutput << ", Phase Setting Sequence = " << asSequence(maxInput) << std::endl;
}

void part2() {}

std::vector<int> read_program() {
  std::vector<int> program_data;
  std::ifstream infile("input.txt");
  while (infile) {
    std::string s;
    if (!getline(infile, s)) {
      break;
    }
    std::istringstream iss(s);
    while (iss) {
      std::string s;
      if (!getline(iss, s, ',')) {
        break;
      }
      int i;
      std::istringstream(s) >> i;
      program_data.push_back(i);
    }
  }
  infile.close();
  return program_data;
}

std::vector<int> phase_sequence_generator() {
  std::vector<int> sequences;
  for (int i = 0; i <= 4; i++) {
    for (int j = 0; j <= 4; j++) {
      for (int k = 0; k <= 4; k++) {
        for (int l = 0; l <= 4; l++) {
          for (int m = 0; m <= 4; m++) {
            if (i==j || i==k || i == l || i==m || j == k || j == l || j == m ||
                k==l || k == m || l == m) {
                continue;
                }
            sequences.push_back(i * 10000 + j * 1000 + k * 100 + l * 10 + m);
          }
        }
      }
    }
  }
  return sequences;
}

int amplify_signal(int phase_sequence, std::vector<int> const tape) {
  std::array<int, 5> phase_seq_array = to_array(phase_sequence);
  VM vm_a(tape);
  VM vm_b(tape);
  VM vm_c(tape);
  VM vm_d(tape);
  VM vm_e(tape);

  int output_amp_0 = amplifier(vm_a, phase_seq_array[0], 0);
  int output_amp_1 = amplifier(vm_b, phase_seq_array[1], output_amp_0);
  int output_amp_2 = amplifier(vm_c, phase_seq_array[2], output_amp_1);
  int output_amp_3 = amplifier(vm_d, phase_seq_array[3], output_amp_2);
  int output_amp_4 = amplifier(vm_e, phase_seq_array[4], output_amp_3);
  return output_amp_4;
}

std::string asSequence(int input) {
  std::array<int, 5> sequenceArray = to_array(input);
  std::stringstream ss;
  ss << "[";
  for (int i=0; i <= 4; i++) {
    ss << sequenceArray[i];
    if (i < 4) {
      ss << ", ";
    }
  }
  ss << "]";
  return ss.str();
}

int amplifier(VM &vm, int phase_setting, int input_signal) {
  vm.output = 0; // watch out
  vm.inputs.push_back(phase_setting);
  vm.inputs.push_back(input_signal);
  run_program(vm, 0);
  return vm.output;
}

void run_program(VM &vm, int pc) {
  vm.pc = pc;
  IntCode instruction;
  while (vm.pc < vm.tape.size()) {
    instruction = read_instruction(vm);
    run_instruction(vm, instruction);
  }
}

IntCode read_instruction(VM &vm) {
  IntCode instruction;
  instruction.modes_n_opcode = vm.tape[vm.pc];
  std::array<int, 5> modes_n_opcode = to_array(vm.tape[vm.pc]);

  instruction.mode_operand_a = modes_n_opcode[2];
  instruction.mode_operand_b = modes_n_opcode[1];
  instruction.mode_operand_c = modes_n_opcode[0];

  instruction.op_code = (modes_n_opcode[3] * 10) + modes_n_opcode[4];

  switch (instruction.op_code) {
  case ADD:
  case MULTIPLY:
    instruction.next_offset = 4;
    instruction.operand_a = instruction.mode_operand_a == IMMEDIATE
                                ? vm.tape[vm.pc + 1]
                                : vm.tape[vm.tape[vm.pc + 1]];
    instruction.operand_b = instruction.mode_operand_b == IMMEDIATE
                                ? vm.tape[vm.pc + 2]
                                : vm.tape[vm.tape[vm.pc + 2]];
    instruction.operand_c = vm.tape[vm.pc + 3];
    break;
  case LESS_THAN:
  case EQUALS:
    instruction.next_offset = 4;
    instruction.operand_a = instruction.mode_operand_a == IMMEDIATE
                                ? vm.tape[vm.pc + 1]
                                : vm.tape[vm.tape[vm.pc + 1]];
    instruction.operand_b = instruction.mode_operand_b == IMMEDIATE
                                ? vm.tape[vm.pc + 2]
                                : vm.tape[vm.tape[vm.pc + 2]];
    instruction.operand_c = vm.tape[vm.pc + 3];
    break;
  case INPUT:
  case OUTPUT:
    instruction.next_offset = 2;
    instruction.operand_a = vm.tape[vm.pc + 1];
    instruction.operand_b = -1;
    instruction.operand_c = -1;
    break;
  case JUMP_IF_TRUE:
  case JUMP_IF_FALSE:
    instruction.next_offset = 3;
    instruction.operand_a = instruction.mode_operand_a == IMMEDIATE
                                ? vm.tape[vm.pc + 1]
                                : vm.tape[vm.tape[vm.pc + 1]];
    instruction.operand_b = instruction.mode_operand_b == IMMEDIATE
                                ? vm.tape[vm.pc + 2]
                                : vm.tape[vm.tape[vm.pc + 2]];
    instruction.operand_c = -1;
    break;
  }
  return instruction;
}

// to_array(45690) => [4, 5, 6, 9, 0]
//                     ↑  ↑  ↑  ↑  ↑
//                     0  1  2  3  4
std::array<int, 5> to_array(int n) {
  std::array<int, 5> result;
  for (int i = 4; i >= 0; i--) {
    int right_most_digit = n % 10;
    result[i] = right_most_digit;
    n /= 10;
  }
  return result;
}

// returns how many steps we need to increase the program counter
void run_instruction(VM &vm,
                     IntCode const &instruction) {
  if (instruction.op_code != ADD && instruction.op_code != MULTIPLY &&
      instruction.op_code != INPUT && instruction.op_code != OUTPUT &&
      instruction.op_code != JUMP_IF_TRUE &&
      instruction.op_code != JUMP_IF_FALSE &&
      instruction.op_code != LESS_THAN && instruction.op_code != EQUALS &&
      instruction.op_code != END) {
    // std::cout << "HALTING, invalid instruction op_code = "
    //           << instruction.op_code << std::endl;
    // std::cout << tape[pc] << "," << tape[pc + 1] << "," << tape[pc + 2] <<
    // ","
    //           << tape[pc + 3] << std::endl;
    vm.pc = vm.tape.size();
    return;
  }

  int operand_a, operand_b, result;
  operand_a = instruction.operand_a;
  operand_b = instruction.operand_b;

  // ADD/MULTIPLY
  if (instruction.op_code == ADD || instruction.op_code == MULTIPLY) {
    if (instruction.op_code == ADD) {
      result = operand_a + operand_b;
    } else if (instruction.op_code == MULTIPLY) {
      result = operand_a * operand_b;
    }
    vm.tape[instruction.operand_c] = result;
  }

  // I/O
  if (instruction.op_code == INPUT) {
    vm.tape[operand_a] = vm.inputs.front();
    vm.inputs.pop_front();
  } else if (instruction.op_code == OUTPUT) {
    vm.output = vm.tape[operand_a];
  }

  // JUMP_IF_TRUE/JUMP_IF_FALSE
  if ((instruction.op_code == JUMP_IF_TRUE && instruction.operand_a != 0) ||
      (instruction.op_code == JUMP_IF_FALSE && instruction.operand_a == 0)) {
    vm.pc = instruction.operand_b;
    return;
  }

  // LESS THAN / EQUALS
  if (instruction.op_code == LESS_THAN) {
    vm.tape[instruction.operand_c] =
        (instruction.operand_a < instruction.operand_b) ? 1 : 0;
  } else if (instruction.op_code == EQUALS) {
    vm.tape[instruction.operand_c] =
        (instruction.operand_a == instruction.operand_b) ? 1 : 0;
  }

  if (instruction.op_code == END) {
    vm.pc = vm.tape.size();
    return;
  }
  vm.pc += instruction.next_offset;
} // run_instruction

void print_opcode_name(int opcode) {
  switch (opcode) {
  case ADD:
    std::cout << "ADD";
    break;
  case MULTIPLY:
    std::cout << "MULTIPLY";
    break;
  case INPUT:
    std::cout << "INPUT";
    break;
  case OUTPUT:
    std::cout << "OUTPUT";
    break;
  case JUMP_IF_TRUE:
    std::cout << "JUMP_IF_TRUE";
    break;
  case JUMP_IF_FALSE:
    std::cout << "JUMP_IF_FALSE";
    break;
  case LESS_THAN:
    std::cout << "LESS_THAN";
    break;
  case EQUALS:
    std::cout << "EQUALS";
    break;
  case END:
    std::cout << "END";
    break;
  }
}