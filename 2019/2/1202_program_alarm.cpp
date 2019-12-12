#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// g++ -std=c++17 1202_program_alarm.cpp -o 1202_program_alarm && ./1202_program_alert
typedef int OpCode;
const OpCode ADD = 1;
const OpCode MULTIPLY = 2;
const OpCode END = 99;

typedef struct IntCode
{
  OpCode op_code;
  int offset_operand_a;
  int offset_operand_b;
  int offset_outout;
} IntCode;

std::vector<int> read_program();
void print_instruction_at(int i, std::vector<int> &tape);
void print_program(std::vector<int> &tape);
IntCode read_instruction(int program_counter, std::vector<int> const &tape);
void run_instruction(IntCode const &instruction, std::vector<int> &tape);
void run_program(std::vector<int> &tape);

int main()
{
  std::vector<int> tape = read_program();
  tape[1] = 12;
  tape[2] = 2;
  print_program(tape);
  run_program(tape);
  print_program(tape);
  std::cout << "Value left @0:" << tape[0] << std::endl << std::endl;
  return 0;
}

std::vector<int> read_program()
{
  std::vector<int> program_data;
  std::ifstream infile("input.txt");
  while (infile)
  {
    std::string s;
    if (!getline(infile, s))
    {
      break;
    }
    std::istringstream iss(s);
    while (iss)
    {
      std::string s;
      if (!getline(iss, s, ','))
      {
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

void print_instruction_at(int i, std::vector<int> &tape)
{
  std::cout << "RAW: offset[" << i << "] " << tape[i] << ", " << tape[i + 1] << ", " << tape[i + 2] << ", " << tape[i + 3] << std::endl;
}

void print_instruction(IntCode instruction, std::vector<int> const &tape)
{
  std::cout << "IntCode: " << instruction.op_code << ", " << instruction.offset_operand_a << ", " << instruction.offset_operand_b << ", " << instruction.offset_outout << std::endl;

  switch (instruction.op_code)
  {
  case ADD:
    std::cout << "ADD @" << instruction.offset_operand_a << ":" << tape[instruction.offset_operand_a] << " + @" << instruction.offset_operand_b << ":" << tape[instruction.offset_operand_b] << " = " << (tape[instruction.offset_operand_a] + tape[instruction.offset_operand_b]) << " into @" << instruction.offset_outout << ":" << tape[instruction.offset_outout] << std::endl;
    break;
  case MULTIPLY:
    std::cout << "MUL @" << instruction.offset_operand_a << ":" << tape[instruction.offset_operand_a] << " * @" << instruction.offset_operand_b << ":" << tape[instruction.offset_operand_b] << " = " << (tape[instruction.offset_operand_a] * tape[instruction.offset_operand_b]) << " into @" << instruction.offset_outout << ":" << tape[instruction.offset_outout] << std::endl;
    break;
  case END:
    std::cout << "END" << std::endl;
    break;
  }
}

void print_program(std::vector<int> &tape)
{
  std::cout << "Size of the tape -> " << tape.size() << std::endl;
  for (int i = 0; i < tape.size(); i += 4)
  {
    print_instruction_at(i, tape);
  }
}

IntCode read_instruction(int program_counter, std::vector<int> &tape)
{
  IntCode instruction;
  instruction.op_code = tape[program_counter];
  instruction.offset_operand_a = tape[program_counter + 1];
  instruction.offset_operand_b = tape[program_counter + 2];
  instruction.offset_outout = tape[program_counter + 3];
  return instruction;
}

void run_instruction(IntCode const &instruction, std::vector<int> &tape)
{  
  switch (instruction.op_code)
  {
  case ADD:
    tape[instruction.offset_outout] = tape[instruction.offset_operand_a] + tape[instruction.offset_operand_b];
    std::cout << "Target After @" << instruction.offset_outout << " -> " << tape[instruction.offset_outout] << std::endl;
    break;
  case MULTIPLY:
    tape[instruction.offset_outout] = tape[instruction.offset_operand_a] * tape[instruction.offset_operand_b];
    std::cout << "Target After @" << instruction.offset_outout << " -> " << tape[instruction.offset_outout] << std::endl;
    break;
  case END:
    std::cout << "HALTING!";
    break;
  }
  std::cout << std::endl;
}

void run_program(std::vector<int> &tape)
{
  int pc = 0;
  IntCode instruction;

  while (true && pc < tape.size())
  {
    std::cout << "PC @ " << pc << std::endl;
    instruction = read_instruction(pc, tape);
    print_instruction(instruction, tape);
    run_instruction(instruction, tape);
    pc += 4;
  }
  std::cout << std::endl
            << "Ended with PC @ " << pc << std::endl << std::endl;
}