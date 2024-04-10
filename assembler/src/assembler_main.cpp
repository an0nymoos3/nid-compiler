/*
 * This is the "main" file. It handles the assembler logic.
 */

#include "assembler.hpp"
#include "exporter.hpp"
#include "lexer.hpp"
#include "utils/commandline.hpp"

int main(int argc, char **argv) {
  Args args = parse_args(argc, argv);
  return 0;
}
