/*
 * This is the "main" file. It handles the assembler logic.
 */

#include "assembler.hpp"
#include "exporter.hpp"
#include <vector>

int main(int argc, char **argv) {
  Args args = parse_args(argc, argv);
  std::vector<Line> lines = parse_file(args);
  lines = tokenize(lines);
  // export_tokens(lines);

  std::vector<AssembeledLine> assembeled_lines = assemble_lines(lines);
  printAssembeledLine(assembeled_lines);

  return 0;
}
