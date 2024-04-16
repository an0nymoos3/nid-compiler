#include "../lexer.hpp"
#include <string>

struct Args {
  bool debug;
  bool terminal_out;
  std::string filename;
};

Args parse_args(int argc, char **argv);

std::vector<Line> parse_file(Args args);
