/*
 * This is the "main" file. It handles the assembler logic.
 */

#include "assembler.hpp"
#include "exporter.hpp"
#include "utils/commandline.hpp"
#include <vector>

int main(int argc, char **argv) {
  Args args = parse_args(argc, argv);
  std::vector<Line> lines = parse_file(args);
  lines = tokenize(lines);
  export_tokens(lines);

  return 0;
}
