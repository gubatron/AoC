// Author: @gubatron December 21, 2019
#include <algorithm>
#include <array>
#include <deque>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

typedef long long bignumber;

typedef int OpCode;
const OpCode ADD = 1;
const OpCode MULTIPLY = 2;
const OpCode INPUT = 3;
const OpCode OUTPUT = 4;
const OpCode JUMP_IF_TRUE = 5;
const OpCode JUMP_IF_FALSE = 6;
const OpCode LESS_THAN = 7;
const OpCode EQUALS = 8;
const OpCode ADJUST_RELATIVE_BASE = 9;
const OpCode END = 99;

typedef int ParameterMode;
const ParameterMode POSITION = 0;
const ParameterMode IMMEDIATE = 1;
const ParameterMode RELATIVE = 2;

typedef struct IntCode {
  int modes_n_opcode;
  OpCode op_code;
  ParameterMode mode_operand_a;
  ParameterMode mode_operand_b;
  ParameterMode mode_operand_c;
  bignumber operand_a;
  bignumber operand_b;
  bignumber operand_c;
  int next_offset;
} IntCode;

typedef struct VM {
  std::string tag;
  int pc;
  int relative_base;
  std::vector<bignumber> tape;
  std::deque<bignumber> inputs;
  bignumber output;
  bool halted;
  bool paused;
  bool pause_on_output;
  bool quiet_output;
  VM(std::string t, std::vector<bignumber> program)
      : tag(t), pc(0), relative_base(0), tape(program), halted(false),
        paused(false), pause_on_output(false), quiet_output(true) {}
  void add_input(bignumber const input) { inputs.push_back(input); }
} VM;

std::array<int, 5> to_array(int n);
std::vector<bignumber> read_program(std::string intcode_program);
std::vector<bignumber> read_program_from_file(std::string filepath);
void init_operand_offsets();
IntCode read_instruction(VM const &tape);
void run_instruction(VM &vm, IntCode const &instruction);
void run_program(VM &VM);
std::string asSequence(int input);
void part1();
void part2();

void test_day7(std::vector<bignumber> tape, bool const feedback_loop_mode,
               bool const quiet_output, bool const pause_on_output,
               std::string expectedOutput);

void test_day7_1();
void test_day7_2();
void test_day7_3();
void test_day7_full(bool const feedback_loop_mode, bool const quiet_output,
                    bool const pause_on_output, std::string expectedOuput);
void test_quine();
void test_big_multiplication();
void test_output_bignumber();

int amplifier(VM &vm, int const input_signal);
int amplify_signal(std::vector<bignumber> const tape, int const phase_sequence,
                   bool const feedback_loop_mode, bool const quiet_output,
                   bool const pause_on_output);
std::vector<int> phase_sequence_generator(bool const feedback_loop_mode);
void print_opcode_name(int opcode);

int main() {
  init_operand_offsets();
  test_day7_1();
  test_day7_2();
  test_day7_3();

  bool feedback_loop_mode = false;
  bool quiet_output=true;
  bool pause_on_output=false;
  test_day7_full(feedback_loop_mode, quiet_output, pause_on_output, "Part 1: Max Thruster Signal = 567045, Phase Setting Sequence = [0, 2, 4, 3, 1]");

  feedback_loop_mode = true;
  quiet_output=true;
  pause_on_output=true;
  test_day7_full(feedback_loop_mode, quiet_output, pause_on_output, "Part 2:  Max Thruster Signal = 39016654, Phase Setting Sequence = [6, 5, 7, 8, 9]");

  test_quine();
  test_big_multiplication();
  test_output_bignumber();
  part1();
  // part2();
  return 0;
}

void test_day7_1() {
  std::vector<bignumber> tape =
      read_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
  bool feedback_loop_mode = false;
  bool quiet_output = true;
  bool pause_on_output = false;
  test_day7(tape, feedback_loop_mode, quiet_output, pause_on_output,
            "Test Day 7/1 43210 - Phase Setting Sequence [4,3,2,1,0]");
}

void test_day7_2() {
  std::vector<bignumber> tape =
      read_program("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,"
                   "23,4,23,99,0,0");
  bool feedback_loop_mode = false;
  bool quiet_output = true;
  bool pause_on_output = false;
  test_day7(tape, feedback_loop_mode, quiet_output, pause_on_output,
            "Test Day 7/2 54321 - Phase Setting Sequence [0,1,2,3,4]");
}

void test_day7_3() {
  std::vector<bignumber> tape =
      read_program("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,"
                   "7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
  bool feedback_loop_mode = false;
  bool quiet_output = true;
  bool pause_on_output = false;
  test_day7(tape, feedback_loop_mode, quiet_output, pause_on_output,
            "Test Day 7/2 65210 - Phase Setting Sequence [1,0,4,3,2]");
}

void test_day7_full(bool const feedback_loop_mode, bool const quiet_output,
                    bool const pause_on_output, std::string expectedOutput) {
  std::vector<bignumber> tape = read_program_from_file("../7/input.txt");
  test_day7(tape, feedback_loop_mode, quiet_output, pause_on_output,
            expectedOutput);
}

void test_day7(std::vector<bignumber> tape, bool feedback_loop_mode,
               bool quiet_output, bool pause_on_output,
               std::string expectedOutput) {
  std::vector<int> sequences = phase_sequence_generator(feedback_loop_mode);
  int maxInput = 0, maxOutput = 0;
  for (auto it = sequences.begin(); it != sequences.end(); it++) {
    int output;
    int sequence = *it;
    if ((output = amplify_signal(tape, sequence, feedback_loop_mode,
                                 quiet_output, pause_on_output)) > maxOutput) {
      maxInput = sequence;
      maxOutput = output;
    }
  }
  std::cout << std::endl << "[E] " << expectedOutput << std::endl;
  std::cout << "    Part " << ((!feedback_loop_mode) ? 1 : 2)
            << ":  Max Thruster Signal = " << maxOutput
            << ", Phase Setting Sequence = " << asSequence(maxInput)
            << std::endl;
}

void test_quine() {
  std::vector<bignumber> tape =
      read_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
  std::cout << std::endl
            << "[E] 109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
            << std::endl;
  VM vm("test_quine", tape);
  vm.quiet_output = false;
  vm.pause_on_output = false;
  run_program(vm);
  std::cout << std::endl;
}

void test_big_multiplication() {
  std::vector<bignumber> tape =
      read_program("1102,34915192,34915192,7,4,7,99,0");
  std::cout << std::endl << "[E] 1219070632396864" << std::endl;
  VM vm("test_big_multiplication", tape);
  vm.quiet_output = false;
  vm.pause_on_output = false;
  run_program(vm);
  std::cout << std::endl;
}

void test_output_bignumber() {
  std::vector<bignumber> tape = read_program("104,1125899906842624,99");
  std::cout << std::endl << "[E] 1125899906842624" << std::endl;
  VM vm("test_big_multiplication", tape);
  vm.quiet_output = false;
  vm.pause_on_output = false;
  run_program(vm);
  std::cout << std::endl;
}

void part1() {
  std::vector<bignumber> tape = read_program_from_file("input.txt");
  std::cout << std::endl << "[E] Day 9/Part 1: 2350741403" << std::endl;
  VM vm("part1", tape);
  vm.quiet_output = false;
  vm.pause_on_output = false;
  vm.add_input(1);
  run_program(vm);
  std::cout << std::endl;
}

std::vector<bignumber> read_program(std::string intcode_program) {
  std::vector<bignumber> program_data;
  std::istringstream iss(intcode_program);
  std::string s;
  while (iss) {
    if (!getline(iss, s, ',')) {
      break;
    }
    bignumber i;
    std::istringstream(s) >> i;
    program_data.push_back(i);
  }
  while (program_data.size() < 4096) {
    program_data.push_back(0);
  }
  return program_data;
}

std::vector<bignumber> read_program_from_file(std::string filepath) {
  std::vector<bignumber> program_data;
  std::ifstream infile(filepath);
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
      bignumber i;

      std::istringstream(s) >> i;
      program_data.push_back(i);
    }
  }
  infile.close();
  while (program_data.size() < 4096) {
    program_data.push_back(0);
  }
  return program_data;
}

std::vector<int> phase_sequence_generator(bool const feedback_loop_mode) {
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

int amplify_signal(std::vector<bignumber> const tape, int const phase_sequence,
                   bool const feedback_loop_mode, bool const quiet_output,
                   bool const pause_on_output) {
  std::array<int, 5> phase_seq_array = to_array(phase_sequence);
  VM vm_a("A", tape);
  VM vm_b("B", tape);
  VM vm_c("C", tape);
  VM vm_d("D", tape);
  VM vm_e("E", tape);

  vm_a.quiet_output = vm_b.quiet_output = vm_c.quiet_output =
      vm_d.quiet_output = vm_d.quiet_output = quiet_output;
  vm_a.pause_on_output = vm_b.pause_on_output = vm_c.pause_on_output =
      vm_d.pause_on_output = vm_e.pause_on_output = pause_on_output;

  vm_a.add_input(phase_seq_array[0]);
  vm_b.add_input(phase_seq_array[1]);
  vm_c.add_input(phase_seq_array[2]);
  vm_d.add_input(phase_seq_array[3]);
  vm_e.add_input(phase_seq_array[4]);
  int signal = 0;
  if (!feedback_loop_mode) {
    signal = amplifier(vm_a, 0);
    signal = amplifier(vm_b, signal);
    signal = amplifier(vm_c, signal);
    signal = amplifier(vm_d, signal);
    signal = amplifier(vm_e, signal);
  } else {
    while (!vm_e.halted) {
      signal = amplifier(vm_a, signal);
      signal = amplifier(vm_b, signal);
      signal = amplifier(vm_c, signal);
      signal = amplifier(vm_d, signal);
      signal = amplifier(vm_e, signal);
    }
    // std::cout << "amplify_signal(phase_sequence=" << phase_sequence
    //            << ") => signal=" << signal << std::endl;
  }
  return signal;
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

std::array<int, 10> op_code_offsets;

void init_operand_offsets() {
    op_code_offsets[0] = 0;
    op_code_offsets[ADD] = 4;
    op_code_offsets[MULTIPLY] = 4;
    op_code_offsets[LESS_THAN] = 4;
    op_code_offsets[EQUALS] = 4;
    op_code_offsets[INPUT] = 2;
    op_code_offsets[OUTPUT] = 2;
    op_code_offsets[ADJUST_RELATIVE_BASE] = 2;
    op_code_offsets[JUMP_IF_TRUE] = 3;
    op_code_offsets[JUMP_IF_FALSE] = 3;
}

void prepare_operand(VM const &vm, int const operand_mode,
                     int const parameter_distance, bool const isOutputParameter,
                     bignumber &operand) {
  bignumber parameter = vm.tape[vm.pc + parameter_distance];

  if (operand_mode == POSITION && !isOutputParameter) {
    operand = vm.tape[parameter];
  } else if (operand_mode == POSITION && isOutputParameter) {
    operand = parameter; // INPUT fits this
  } else if (operand_mode == IMMEDIATE && !isOutputParameter) {
    operand = parameter;
  } else if (operand_mode == IMMEDIATE && isOutputParameter) {
    // "Parameters that an instruction writes to will never be in immediate
    // mode." -Problem 5. Therefore we force IMMEDIATE to act like POSITION
    // mode.
    operand = vm.tape[parameter];
  } else if (operand_mode == RELATIVE && !isOutputParameter) {
    operand = vm.tape[vm.relative_base + parameter];
  } else if (operand_mode == RELATIVE && isOutputParameter) {
    operand = vm.relative_base + parameter; // INPUT fits this
  }
}

IntCode read_instruction(VM const &vm) {
  IntCode instruction;
  instruction.modes_n_opcode = vm.tape[vm.pc];
  std::array<int, 5> modes_n_opcode = to_array(vm.tape[vm.pc]);

  instruction.mode_operand_a = modes_n_opcode[2];
  instruction.mode_operand_b = modes_n_opcode[1];
  instruction.mode_operand_c = modes_n_opcode[0];

  instruction.op_code = (modes_n_opcode[3] * 10) + modes_n_opcode[4];
  instruction.next_offset = op_code_offsets[instruction.op_code];
  
  instruction.operand_a = -1;
  instruction.operand_b = -1;
  instruction.operand_c = -1;

  const bool isOutputParameter = true;
  const bool isInputParameter = false;

  switch (instruction.op_code) {
  case ADD: //01
  case MULTIPLY: //02
  case LESS_THAN: //07
  case EQUALS: //08
    prepare_operand(vm, instruction.mode_operand_a, 1, isInputParameter,
                    instruction.operand_a);
    prepare_operand(vm, instruction.mode_operand_b, 2, isInputParameter,
                    instruction.operand_b);
    prepare_operand(vm, instruction.mode_operand_c, 3, isOutputParameter,
                    instruction.operand_c);
    break;
  case JUMP_IF_TRUE: //05
  case JUMP_IF_FALSE: // 06
    prepare_operand(vm, instruction.mode_operand_a, 1, isInputParameter,
                    instruction.operand_a);
    prepare_operand(vm, instruction.mode_operand_b, 2, isInputParameter,
                    instruction.operand_b);
    break;
  case INPUT: //03
    prepare_operand(vm, instruction.mode_operand_a, 1, isOutputParameter, instruction.operand_a);
    break;
  case OUTPUT: //04
  case ADJUST_RELATIVE_BASE: //09
    prepare_operand(vm, instruction.mode_operand_a, 1,
                    isInputParameter, instruction.operand_a);
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
      instruction.op_code != ADJUST_RELATIVE_BASE &&
      instruction.op_code != END) {
    //    std::cout << "VM " << vm.tag << ": HALTING, invalid instruction
    //    op_code = "
    //              << instruction.op_code << std::endl;
    //    std::cout << "PC@" << vm.pc << std::endl;
    //    std::cout << vm.tape[vm.pc] << "," << vm.tape[vm.pc + 1] << ","
    //              << vm.tape[vm.pc + 2] << "," << vm.tape[vm.pc + 3] <<
    //              std::endl;
    vm.pc = vm.tape.size();
    vm.halted = true;
    return;
  }

  bignumber operand_a, operand_b, operand_c, result;
  operand_a = instruction.operand_a;
  operand_b = instruction.operand_b;
  operand_c = instruction.operand_c;

  // ADD/MULTIPLY
  if (instruction.op_code == ADD || instruction.op_code == MULTIPLY) {
    if (instruction.op_code == ADD) {
      result = operand_a + operand_b;
    } else if (instruction.op_code == MULTIPLY) {
      result = operand_a * operand_b;
    }
    vm.tape[operand_c] = result;
  }

  // I/O
  if (instruction.op_code == INPUT) {
    if (vm.inputs.size() == 0) {
      throw std::runtime_error("vm inputs should never be 0 sized");
    }
    vm.tape[operand_a] = vm.inputs.front();
    if (vm.inputs.size() > 0) {
      vm.inputs.pop_front();
    }

  } else if (instruction.op_code == OUTPUT) {
    vm.output = operand_a;
    if (!vm.quiet_output) {
      std::cout << vm.output << " ";
      std::flush(std::cout);
    }
    if (vm.pause_on_output) {
      vm.paused = true;
    }
  }

  // JUMP_IF_TRUE/JUMP_IF_FALSE
  if ((instruction.op_code == JUMP_IF_TRUE && instruction.operand_a != 0) ||
      (instruction.op_code == JUMP_IF_FALSE && instruction.operand_a == 0)) {
    vm.pc = operand_b;
    return;
  }

  // LESS THAN / EQUALS
  if (instruction.op_code == LESS_THAN) {
    vm.tape[operand_c] = (operand_a < operand_b) ? 1 : 0;
  } else if (instruction.op_code == EQUALS) {
    vm.tape[operand_c] = (operand_a == operand_b) ? 1 : 0;
  }

  if (instruction.op_code == ADJUST_RELATIVE_BASE) {
    vm.relative_base += operand_a;
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