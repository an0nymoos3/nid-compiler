#include "../lexer.hpp"
#include <string>

struct Args {
  bool debug;
  std::string filename; // Input filename
  std::string outname;  // Output filename
};

Args parse_args(int argc, char **argv);

std::vector<Line> parse_file(Args args);
