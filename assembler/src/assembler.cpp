/*
 * This file is for assembling or converting from regular english to binary.
 */
#include "assembler.hpp"
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

  // Pre-calculate to which lines we will jmp to
  for (int i = 0; i < lines.size(); i++) {
    if (lines[i].line_tokens.size() != 1) {
      for (int j = 0; j < lines[i].line_tokens.size(); j++) {
        if (lines[i].line_tokens[j].token_type == JmpPoint) {
          jmp_map[lines[i].line_tokens[j].value] = i + 1;
        }
      }
    }
  }

  for (int i = 0; i < lines.size(); i++) {
    AssembeledLine line = assemble_line(lines[i], jmp_map);
    line.error_code = lines[i].error_code;
    if (line.line_content.size() != 0) {
      line.line_number = i + 1;
      assembeled_lines.push_back(line);
    }
  }

  return assembeled_lines;
}

AssembeledLine assemble_line(Line line, std::map<std::string, int> jmp_map) {
  AssembeledLine ass_line;
  std::string ass_string;

  for (Token token : line.line_tokens) {
    if (token.token_type == EOL || token.token_type == Comment) {
      ass_string += "\n";
      ass_string = binary_to_hex(ass_string);
      ass_line = {ass_string, line.line_number};
      return ass_line;
    } else if (token.token_type == Operation) {
      ass_string +=
          operation_to_binary(token.value, line.line_number, line.error_code);
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

  std::cout << "Warning: Line did not end with EOL character!" << std::endl;
  return ass_line;
}

std::string operation_to_binary(std::string value, int line_number,
                                int &error_code) {
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
  }

  std::cout << "Error 2: Unknown operation at line " << line_number
            << "\nOperation " << value << " used, parsed as NOP" << std::endl
            << std::endl;
  error_code = 2;

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
  std::cout << "Printing lines as hex code: " << std::endl;

  for (int i = 0; i < lines.size(); i++) {
    if (lines[i].error_code == 0) {
      std::cout << "x\"" << lines[i].line_content << "\", " << std::endl;
    } else {
      std::cout << "x\"" << lines[i].line_content
                << "\", \t --Line error: " << lines[i].error_code << std::endl;
    }
  }
}
