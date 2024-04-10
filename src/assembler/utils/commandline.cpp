#include "commandline.hpp"
#include <cstddef>
#include <cstdlib>
#include <cstring>
#include <iostream>

Args parse_args(int argc, char **argv) {
  Args args;
  args.debug = false;
  args.terminal_out = false;

  for (int i = 1; i < argc; i++) {

    if (std::strstr(argv[i], ".ass") != NULL) {
      args.filename = argv[i];
    }

    if (argv[i] == "-d" || argv[i] == "--debug") {
      args.debug = true;
    }

    if (argv[i] == "-t" || argv[i] == "--output-terminal") {
      args.terminal_out = true;
    }
  }

  if (args.filename.empty()) {
    std::cout << "No .ass file provided!" << std::endl;
    exit(1);
  }

  return args;
}
