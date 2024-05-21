/*
 * This is the "main" file. It handles the assembler logic.
 */

#include "exporter.hpp"
#include <iostream>
#include <vector>

int main(int argc, char **argv) {
  Args args = parse_args(argc, argv);
  bool assembly_failed = false;
  std::vector<Line> lines = parse_file(args);
  lines = tokenize(lines, assembly_failed);

  if (args.debug) {
    for (const Line &line : lines) {
      for (const Token &token : line.line_tokens) {
        std::cout << token.value << std::endl;
      }
    }
  }

  std::vector<AssembeledLine> assembeled_lines =
      assemble_lines(lines, assembly_failed);

  if (assembly_failed) {
    std::cout << "Assembly failed! Please fix the errors/warnings above and "
                 "try again!"
              << std::endl;
    return 1;
  }

  if (args.debug) {
    printAssembeledLine(assembeled_lines);
  }

  write_to_file(assembeled_lines, args.outname); // Write binary program to file

  std::cout << "Assembly complete! \nWritten to: " << args.outname << std::endl;

  return 0;
}
