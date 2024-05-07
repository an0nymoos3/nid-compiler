/*
 * This file is for assembling or converting from regular english to binary.
 */
#include "utils/commandline.hpp"
#include <map>
#include <string>
#include <vector>

struct AssembeledLine {
  std::string line_content;
  int line_number;
};

std::vector<AssembeledLine> assemble_lines(std::vector<Line> lines,
                                           bool &assembly_failed);

AssembeledLine assemble_line(Line line, std::map<std::string, int> jmp_map,
                             bool &assembly_failed);

void printAssembeledLine(std::vector<AssembeledLine> lines);

std::string operation_to_binary(std::string value, int line_number,
                                bool &assembly_failed);

std::string binary_to_hex(std::string binary_string);
