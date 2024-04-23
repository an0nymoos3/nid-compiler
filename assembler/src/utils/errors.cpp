#include "errors.hpp"
#include <iostream>

void print_error(Error &err) {
  std::cout << err.err_msg << std::endl;
  std::cout << " " << err.line_num - 1 << " | " << std::endl;
  std::cout << " " << err.line_num << " | " << err.line_content << std::endl;
  std::cout << " " << err.line_num + 1 << " | " << std::endl;

  // Break up between lines
  std::cout << std::endl;
  std::cout << " --------------------------------------- " << std::endl;
  std::cout << std::endl;
}
