/*
 * This is the "main" file. It handles the assembler logic.
 */

#include "assembler.hpp"
#include "exporter.hpp"
#include <iostream>
#include <vector>

int main(int argc, char **argv) {
  Args args = parse_args(argc, argv);
  bool assembly_failed = false;
  std::vector<Line> lines = parse_file(args);
  lines = tokenize(lines, assembly_failed);
  // export_tokens(lines); // For debugging

  std::vector<AssembeledLine> assembeled_lines =
      assemble_lines(lines, assembly_failed);

  if (assembly_failed) {
    std::cout << "Assembly failed! Please fix the errors/warnings above and "
                 "try again!"
              << std::endl;
    return 1;
  }

  printAssembeledLine(assembeled_lines);

  return 0;
}
