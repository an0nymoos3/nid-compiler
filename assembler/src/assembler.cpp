/*
 * This file is for assembling or converting from regular english to binary.
 */
#include "assembler.hpp"
#include "utils/errors.hpp"
#include <algorithm>
#include <bitset>
#include <ios>
#include <iostream>
#include <map>
#include <sstream>
#include <vector>

std::vector<AssembeledLine> assemble_lines(std::vector<Line> lines) {
  std::vector<AssembeledLine> assembeled_lines;
  std::map<std::string, int> jmp_map;

  int non_empty_lines = 0;
  // Pre-calculate to which lines we will jmp to
  for (int i = 0; i < lines.size(); i++) {
    if (lines[i].line_tokens.size() != 1) {
      non_empty_lines++;
      for (int j = 0; j < lines[i].line_tokens.size(); j++) {
        if (lines[i].line_tokens[j].token_type == JmpPoint) {
          // -1 to give correct line after fecthing assembly instruction
          jmp_map[lines[i].line_tokens[j].value] = non_empty_lines - 1;

          // Remove lines only containing JmpPoints
          if (j == 0 && lines[i].line_tokens.size() <= 2) {
            non_empty_lines--;
          }
        }
      }
    }
  }

  for (int i = 0; i < lines.size(); i++) {
    AssembeledLine line = assemble_line(lines[i], jmp_map, assembly_failed);
    if (line.line_content.size() != 0) {
      line.line_number = i + 1;
      assembeled_lines.push_back(line);
    }
  }

  return assembeled_lines;
}

AssembeledLine assemble_line(Line line, std::map<std::string, int> jmp_map,
                             bool &assembly_failed) {
  AssembeledLine ass_line;
  std::string ass_string;

  for (Token token : line.line_tokens) {
    if (token.token_type == EOL || token.token_type == Comment) {
      // ass_string = binary_to_hex(ass_string);
      ass_line = {ass_string, line.line_number};
      return ass_line;
    } else if (token.token_type == Operation) {
      ass_string +=
      operation_to_binary(token.value, line.line_number, assembly_failed);
    } else if (token.token_type == Mode || token.token_type == Register ||
               token.token_type == Constant) {
      ass_string += token.value;
    } else if (token.token_type == JmpOP) {
      // Convert the decimal linenumber to binary string
      std::bitset<sizeof(int) * 4> bits(jmp_map[token.value]);
      std::ostringstream oss;
      oss << bits;
      std::string value = oss.str();
      ass_string += value;
    }
  }
  Error err = {line.line_number,
               "Warning: Line did not end with EOL character!",
               line.line_content};
  print_error(err);
  return ass_line;
}

std::string operation_to_binary(std::string value, int line_number,
                                bool &assembly_failed) {

  // Convert the entire string to uppercase using std::transform()
  std::transform(value.begin(), value.end(), value.begin(), ::toupper);

  if (value == "NOP") {
    return "000000";
  } else if (value == "LDI") {
    return "000001";
  } else if (value == "LD") {
    return "000010";
  } else if (value == "ST") {
    return "000011";
  } else if (value == "PSH") {
    return "000100";
  } else if (value == "POP") {
    return "000101";
  } else if (value == "ADD") {
    return "000110";
  } else if (value == "ADDI") {
    return "000111";
  } else if (value == "SUB") {
    return "001000";
  } else if (value == "SUBI") {
    return "001001";
  } else if (value == "MUL") {
    return "001010";
  } else if (value == "MULI") {
    return "001011";
  } else if (value == "CMP") {
    return "001100";
  } else if (value == "CMPI") {
    return "001101";
  } else if (value == "AND") {
    return "001110";
  } else if (value == "ANDI") {
    return "001111";
  } else if (value == "OR") {
    return "010000";
  } else if (value == "ORI") {
    return "010001";
  } else if (value == "JMP") {
    return "010010";
  } else if (value == "JSR") {
    return "010011";
  } else if (value == "RET") {
    return "010100";
  } else if (value == "BEQ") {
    return "010101";
  } else if (value == "BNE") {
    return "010110";
  } else if (value == "BPL") {
    return "010111";
  } else if (value == "BMI") {
    return "011000";
  } else if (value == "BGE") {
    return "011001";
  } else if (value == "BLT") {
    return "011010";
  } else if (value == "MVIX") {
    return "011011";
  } else if (value == "MVIY") {
    return "011100";
  } else if (value == "MVIIX") {
    return "011101";
  } else if (value == "MVIIY") {
    return "011110";
  } else if (value == "KBD") {
    return "011111";
  } else if (value == "BYK") {
    return "100000";
  } else if (value == "BNK") {
    return "100001";
  } else if (value == "ADDIX") {
    return "100010";
  } else if (value == "ADDIY") {
    return "100011";
  } else if (value == "ADDIIX") {
    return "100100";
  } else if (value == "ADDIIY") {
    return "100101";
  } else if (value == "SUPI") {
    return "100110";
  } else if (value == "SDWI") {
    return "100111";
  } else if (value == "SUPII") {
    return "101000";
  } else if (value == "SDWII") {
    return "101001";
  } else if (value == "LFLIP") {
    return "101010";
  } else if (value == "RFLIP") {
    return "101011";
  }

  Error err = {line_number,
               "Error: Unkown assembly operation! For more info on what "
               "operations are supported: "
               "https://github.com/an0nymoos3/nid-compiler/blob/assembler/docs/"
               "assembly.md",
               value};
  print_error(err);
  assembly_failed = true;

  return "000000";
}

std::string binary_to_hex(std::string binary_string) {
  if (binary_string.size() == 1) {
    return "";
  }

  // Convert binary string to an integer
  int decimalNumber = std::stoi(binary_string, nullptr, 2);

  // Convert integer to hexadecimal string
  std::stringstream ss;
  ss << std::uppercase << std::hex << decimalNumber;
  std::string hexadecimalString = ss.str();

  while (hexadecimalString.size() < 7) {
    hexadecimalString = "0" + hexadecimalString;
  }

  return hexadecimalString;
}

void printAssembeledLine(std::vector<AssembeledLine> lines) {
  std::cout << "Resulting binary code: " << std::endl;

  for (int i = 0; i < lines.size(); i++) {
    std::cout << lines[i].line_content << std::endl;
  }
}
