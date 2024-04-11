/*
 * This file is for assembling or converting from regular english to binary.
 */
#include "assembler.hpp"
#include <algorithm>
#include <ios>
#include <iostream>
#include <sstream>
#include <vector>

std::vector<AssembeledLine> assemble_lines(std::vector<Line> lines) {
  std::vector<AssembeledLine> assembeled_lines;

  for (int i = 0; i < lines.size(); i++) {
    AssembeledLine line = assemble_line(lines[i]);
    if (line.line_content.size() != 0) {
      line.line_number = i + 1;
      assembeled_lines.push_back(line);
    }
  }

  return assembeled_lines;
}

AssembeledLine assemble_line(Line line) {
  AssembeledLine ass_line;
  std::string ass_string;

  for (Token token : line.line_tokens) {
    if (token.token_type == EOL || token.token_type == Comment) {
      ass_string += "\n";
      ass_string = binary_to_hex(ass_string);
      ass_line = {ass_string, line.line_number};
      return ass_line;
    } else if (token.token_type == Operation) {
      ass_string += operation_to_binary(token.value);
    } else if (token.token_type == Mode || token.token_type == Register ||
               token.token_type == Constant) {
      ass_string += token.value;
    }
  }

  return ass_line;
}

std::string operation_to_binary(std::string value) {
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

  std::cout << "Unknown operation used, parsed as NOP" << std::endl;
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
    std::cout << "x\"" << lines[i].line_content << "\", " << std::endl;
  }
}
