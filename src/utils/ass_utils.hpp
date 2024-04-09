#include <string>

struct Line {
  int linenr;
  std::string content;
};

struct Error {
  struct Line line;
  std::string error_msg;
};

/**
 * Does a pretty print of an Error.
 */
void print_error(Error error);
