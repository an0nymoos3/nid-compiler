/*
 * This is the "main" file. It handles the assembler logic.
 */

#include "assembler.hpp"
#include "exporter.hpp"
#include "lexer.hpp"
#include "utils/commandline.hpp"
#include <vector>

int main(int argc, char **argv) {
  // Args args = parse_args(argc, argv);
  std::vector<Line> lines;

  std::vector<Token> line_tokens;
  Line line1 = {line_tokens, "NOP, 00, 0000, 0 \n", 1};
  lines.push_back(line1);

  std::vector<Token> line_tokens2;
  Line line2 = {line_tokens2, "ST, 10, 1101, 486; This is a comment \n", 2};
  lines.push_back(line2);

  std::vector<Token> line_tokens3;
  Line line3 = {line_tokens3, "LDI, 00, 0011, 7 \n", 3};
  lines.push_back(line3);

  lines = tokenize(lines);
  export_tokens(lines);

  return 0;
}
