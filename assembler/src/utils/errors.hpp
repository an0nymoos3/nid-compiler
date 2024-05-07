#include <string>

struct Error {
  int line_num;
  std::string err_msg;
  std::string line_content;
};

void print_error(Error &err);
