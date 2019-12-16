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

std::array<int, 5> to_array(int n);
std::vector<int> read_program();
IntCode read_instruction(int program_counter, std::vector<int> const &tape);
void run_instruction(int pc, int input, IntCode const &instruction, std::vector<int> &tape);
void run_program(int input, std::vector<int> &tape);
void part1();
void part2();

int main() {
  part1();
  return 0;
}

void part1() {
  std::vector<int> tape = read_program();
  // print_program(tape);
  run_program(1, tape);
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

std::array<int, 5> to_array(int n) {
  std::array<int, 5> result;
  for (int i = 4; i >= 0; i--) {
    int right_most_digit = n % 10;
    result[i] = right_most_digit;
    n /= 10;
  }
  return result;
}

IntCode read_instruction(int pc, std::vector<int> const &tape) {
      std::cout << "PC @ " << pc << std::endl;
  IntCode instruction;
  instruction.modes_n_opcode = tape[pc];
  std::array<int, 5> modes_n_opcode = to_array(tape[pc]);

  std::cout << "modes_n_opcode = " 
            << instruction.modes_n_opcode << " -> ["
            << modes_n_opcode[0] << " " << modes_n_opcode[1] << " "
            << modes_n_opcode[2] << " " << modes_n_opcode[3] << " "
            << modes_n_opcode[4] << "]" << std::endl;

  instruction.mode_operand_a = modes_n_opcode[2];
  instruction.mode_operand_b = modes_n_opcode[1];
  instruction.mode_operand_c = modes_n_opcode[0];

  instruction.op_code = (modes_n_opcode[3] * 10) + modes_n_opcode[4];
  
  switch (instruction.op_code) {
  case ADD:
  case MULTIPLY:    
    std::cout << "(RAW)       " << tape[pc] << "," << tape[pc+1] << "," << tape[pc+2] << "," << tape[pc+3] << std::endl;
    instruction.next_offset = 4;
    instruction.operand_a = instruction.mode_operand_a == IMMEDIATE ? tape[pc + 1] : tape[tape[pc + 1]];
    instruction.operand_b = instruction.mode_operand_b == IMMEDIATE ? tape[pc + 2] : tape[tape[pc + 2]] ;
    instruction.operand_c = tape[pc + 3];
    std::cout << "(PROCESSED) " << tape[pc] << "," << instruction.operand_a << "," << instruction.operand_b << "," << instruction.operand_c << std::endl;
    break;
  case INPUT:
  case OUTPUT:
    std::cout << "(RAW)       " << tape[pc] << "," << tape[pc+1] << std::endl;
    instruction.next_offset = 2;
    instruction.operand_a = tape[pc + 1];
    instruction.operand_b = -1;
    instruction.operand_c = -1;
    std::cout << "(PROCESSED)       " << tape[pc] << ", @" << tape[pc+1] << ":" << tape[instruction.operand_a] << std::endl;
    break;
  }
  return instruction;
}

void run_instruction(int pc, int input, IntCode const &instruction,
                     std::vector<int> &tape) {
  if (instruction.op_code != ADD && instruction.op_code != MULTIPLY && instruction.op_code != INPUT && instruction.op_code != OUTPUT && instruction.op_code != END) {
    std::cout << "HALTING, invalid instruction op_code = " << instruction.op_code << std::endl;
    std::cout << tape[pc] << "," << tape[pc+1] << "," << tape[pc+2] << "," << tape[pc+3] << std::endl;
    exit(-1);
    return;
  }

  int operand_a, operand_b, result;
  operand_a = instruction.operand_a;
  operand_b = instruction.operand_b;

  if (instruction.op_code == ADD || instruction.op_code == MULTIPLY) {        
    if (instruction.op_code == ADD) {      
      result = operand_a + operand_b;
      std::cout << "ADD: " << operand_a << " + " << operand_b << " = " << result << std::endl;
    } else if (instruction.op_code == MULTIPLY) {
      result = operand_a * operand_b;
      std::cout << "MUL: " << operand_a << " * " << operand_b << " = " << result << std::endl;
    }
    std::cout << "Storing result: " << result << " @ " << instruction.operand_c << std::endl;
    std::cout << "BEFORE: @" << instruction.operand_c << ":" << tape[instruction.operand_c] << std::endl;
    tape[instruction.operand_c] = result;
    std::cout << "AFTER: @" << instruction.operand_c << ":" << tape[instruction.operand_c] << std::endl;
  }

  if (instruction.op_code == INPUT) {
    std::cout << tape[pc] << "," << tape[pc+1] << std::endl;
    std::cout << "INPUT: input=" << input << " into @" << operand_a << std::endl;
    std::cout << "BEFORE: @" << operand_a << ":" << tape[operand_a] << std::endl;
    tape[operand_a] = input;
    std::cout << "AFTER: @" << operand_a << ":" << tape[operand_a] << std::endl;
  } else if (instruction.op_code == OUTPUT) {
    std::cout << tape[pc] << "," << tape[pc+1] << std::endl;
    std::cout << "OUTPUT: value @" << operand_a << " => " << tape[operand_a] << std::endl;
  }

  if (instruction.op_code == END) {
    std::cout << "HALT!" << std::endl;
    exit(0);
  }
  std::cout << std::endl;
}

void run_program(int input, std::vector<int> &tape) {
  int pc = 0;
  IntCode instruction;

  while (pc < tape.size()) {
    instruction = read_instruction(pc, tape);
    run_instruction(pc, input, instruction, tape);
    pc += instruction.next_offset;
  }
  std::cout << std::endl << "Ended with PC @ " << pc << std::endl << std::endl;
}
