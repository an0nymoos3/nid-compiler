#include "errors.hpp"
#include <iostream>

void print_error(Error &err) {
  std::cout << err.err_msg << std::endl;
  std::cout << " " << err.line_num - 1 << " | " << std::endl;
  std::cout << " " << err.line_num << " | " << err.line_content;

  if (err.line_content.find("\n\r") != std::string::npos) {
    std::cout << std::endl;
  }

  std::cout << " " << err.line_num + 1 << " | " << std::endl;

  // Break up between errors
  std::cout << std::endl;
  std::cout << " --------------------------------------- " << std::endl;
  std::cout << std::endl;
}
