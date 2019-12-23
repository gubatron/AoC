// Author: @gubatron December 21, 2019
#include <algorithm>
#include <array>
#include <deque>
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
  char tag;
  int pc;
  std::vector<int> tape;
  std::deque<int> inputs;
  int output;
  bool halted;
  bool paused;
  VM(char t, std::vector<int> program, int const phase_setting)
      : tag(t), pc(0), tape(program), halted(false), paused(false) {
    inputs.push_back(phase_setting);
  }
} VM;

std::array<int, 5> to_array(int n);
std::vector<int> read_program();
IntCode read_instruction(VM &tape);
void run_instruction(VM &vm, IntCode const &instruction);
void run_program(VM &VM);
std::string asSequence(int input);
void part1();
void part2();
int amplifier(VM &vm, int const input_signal);
int amplify_signal(bool const feedback_loop_mode, int const sequence,
                   std::vector<int> const tape);
std::vector<int> phase_sequence_generator(bool feedback_loop_mode);

int main() {
  part1();
  part2();
  return 0;
}

void part1() {
  std::vector<int> tape = read_program();
  std::vector<int> sequences = phase_sequence_generator(false);
  int maxInput = 0, maxOutput = 0;
  for (auto it = sequences.begin(); it != sequences.end(); it++) {
    int output;
    if ((output = amplify_signal(false, *it, tape)) > maxOutput) {
      maxInput = *it;
      maxOutput = output;
    }
  }
  std::cout << "Part 1:  Max Thruster Signal = " << maxOutput
            << ", Phase Setting Sequence = " << asSequence(maxInput)
            << std::endl;
}

void part2() {
  std::vector<int> tape = read_program();
  std::vector<int> sequences = phase_sequence_generator(true);
  int maxInput = 0, maxOutput = 0;
  for (auto it = sequences.begin(); it != sequences.end(); it++) {
    int output;
    if ((output = amplify_signal(true, *it, tape)) > maxOutput) {
      maxInput = *it;
      maxOutput = output;
    }
  }
  std::cout << "Part 2:  Max Thruster Signal = " << maxOutput
            << ", Phase Setting Sequence = " << asSequence(maxInput)
            << std::endl;
}

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

std::vector<int> phase_sequence_generator(bool feedback_loop_mode) {
  std::vector<int> sequences;
  int feedback_loop_offset = (feedback_loop_mode) ? 5 : 0;
  for (int i = feedback_loop_offset; i <= 4 + feedback_loop_offset; i++) {
    for (int j = feedback_loop_offset; j <= 4 + feedback_loop_offset; j++) {
      for (int k = feedback_loop_offset; k <= 4 + feedback_loop_offset; k++) {
        for (int l = feedback_loop_offset; l <= 4 + feedback_loop_offset; l++) {
          for (int m = feedback_loop_offset; m <= 4 + feedback_loop_offset;
               m++) {
            if (i == j || i == k || i == l || i == m || j == k || j == l ||
                j == m || k == l || k == m || l == m) {
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

int amplify_signal(bool const feedback_loop_mode, int const phase_sequence,
                   std::vector<int> const tape) {
  std::array<int, 5> phase_seq_array = to_array(phase_sequence);
  VM vm_a('A', tape, phase_seq_array[0]);
  VM vm_b('B', tape, phase_seq_array[1]);
  VM vm_c('C', tape, phase_seq_array[2]);
  VM vm_d('D', tape, phase_seq_array[3]);
  VM vm_e('E', tape, phase_seq_array[4]);

  if (!feedback_loop_mode) {
    int signal = 0;
    signal = amplifier(vm_a, 0);
    signal = amplifier(vm_b, signal);
    signal = amplifier(vm_c, signal);
    signal = amplifier(vm_d, signal);
    signal = amplifier(vm_e, signal);
    return signal;
  } else {
    // FEEDBACK LOOP MODE
    int signal = 0;
    while (!vm_e.halted) {
      signal = amplifier(vm_a, signal);
      signal = amplifier(vm_b, signal);
      signal = amplifier(vm_c, signal);
      signal = amplifier(vm_d, signal);
      signal = amplifier(vm_e, signal);
    }
    // std::cout << "amplify_signal(phase_sequence=" << phase_sequence
    //           << ") => signal=" << signal << std::endl;
    return signal;
  }
}

std::string asSequence(int input) {
  std::array<int, 5> sequenceArray = to_array(input);
  std::stringstream ss;
  ss << "[";
  for (int i = 0; i <= 4; i++) {
    ss << sequenceArray[i];
    if (i < 4) {
      ss << ", ";
    }
  }
  ss << "]";
  return ss.str();
}

int amplifier(VM &vm, int const input_signal) {
  vm.inputs.push_back(input_signal);
  run_program(vm);
  return vm.output;
}

void run_program(VM &vm) {
  if (vm.halted) {
    std::cout << "run_program() aborted. Amp " << vm.tag
              << " was already halted!" << std::endl;
    return;
  }
  IntCode instruction;
  vm.paused = false;
  while (vm.pc < vm.tape.size()) {
    instruction = read_instruction(vm);
    run_instruction(vm, instruction);
    if (vm.halted || vm.paused) {
      break;
    }
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
void run_instruction(VM &vm, IntCode const &instruction) {
  if (instruction.op_code != ADD && instruction.op_code != MULTIPLY &&
      instruction.op_code != INPUT && instruction.op_code != OUTPUT &&
      instruction.op_code != JUMP_IF_TRUE &&
      instruction.op_code != JUMP_IF_FALSE &&
      instruction.op_code != LESS_THAN && instruction.op_code != EQUALS &&
      instruction.op_code != END) {
    std::cout << "HALTING, invalid instruction op_code = "
              << instruction.op_code << std::endl;
    std::cout << vm.tape[vm.pc] << "," << vm.tape[vm.pc + 1] << ","
              << vm.tape[vm.pc + 2] << "," << vm.tape[vm.pc + 3] << std::endl;
    vm.pc = vm.tape.size();
    vm.halted = true;
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
    if (vm.inputs.size() == 0) {
      throw std::runtime_error("vm inputs should never be 0 sized");
    }
    vm.tape[operand_a] = vm.inputs.front();
    // std::cout << "VM " << vm.tag << " INPUT <= " << vm.tape[operand_a] <<
    // std::endl;
    if (vm.inputs.size() > 0) {
      vm.inputs.pop_front();
    }

  } else if (instruction.op_code == OUTPUT) {
    vm.output = vm.tape[operand_a];
    // std::cout << "VM " << vm.tag << " OUTPUT <= " << vm.output << std::endl;
    vm.paused = true;
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
    // std::cout << "halting on END instruction" << std::endl;
    vm.halted = true;
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