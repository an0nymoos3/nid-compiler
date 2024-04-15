/*
 * This file is for assembling or converting from regular english to binary.
 */
#include "utils/commandline.hpp"
#include <string>
#include <vector>

struct AssembeledLine {
  std::string line_content;
  int line_number;
  int error_code;
};

std::vector<AssembeledLine> assemble_lines(std::vector<Line> lines);

AssembeledLine assemble_line(Line line);

void printAssembeledLine(std::vector<AssembeledLine> lines);

std::string operation_to_binary(std::string value, int line_number,
                                int &error_code);

std::string binary_to_hex(std::string binary_string);
