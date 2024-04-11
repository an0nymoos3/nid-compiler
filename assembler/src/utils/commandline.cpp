#include "commandline.hpp"
#include <cstddef>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

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

std::vector<Line> parse_file(Args args) {
  // Open the file for reading
  std::ifstream file(args.filename);

  if (!file.is_open()) {
    std::cout << "Error opening the file." << std::endl;
  }

  // Read the file contents into a stringstream
  std::stringstream buffer;
  buffer << file.rdbuf();

  // Close the file
  file.close();

  // Extract the string from the stringstream
  std::string file_contents = buffer.str();

  // Output the string
  std::cout << "File Contents:\n" << file_contents << std::endl;

  // Split the string into lines
  std::vector<Line> lines;
  std::istringstream iss(file_contents);
  std::string one_line;
  int line_number = 1;
  while (std::getline(iss, one_line)) {
    std::vector<Token> tokens;
    Line line = {tokens, one_line + '\n', line_number};
    lines.push_back(line);
    line_number++;
  }

  return lines;
}
