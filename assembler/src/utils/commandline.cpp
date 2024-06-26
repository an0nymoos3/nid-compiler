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

  for (int i = 1; i < argc; i++) {

    // Find input file
    if (std::strstr(argv[i], ".ass") != NULL) {
      args.filename = argv[i];
    }

    // Find optional output name
    if (strcmp(argv[i], "-o") == 0 || strcmp(argv[i], "--output") == 0) {
      args.outname =
          argv[i +
               1]; // Assume that user sends the outname after the output flag
    }

    // Check for debug flag
    if (strcmp(argv[i], "-d") == 0 || strcmp(argv[i], "--debug") == 0) {
      args.debug = true;
    }
  }

  if (args.filename.empty()) {
    std::cout << "No .ass file provided!" << std::endl;
    exit(1);
  }

  if (args.outname.empty()) {
    args.outname = args.filename;
    args.outname.replace(args.outname.end() - 4, args.outname.end(), ".out");
  }

  return args;
}

std::vector<Line> parse_file(Args args) {
  // Open the file for reading
  std::ifstream file(args.filename);

  if (!file.is_open()) {
    std::cout << "Error opening the file." << std::endl;
    exit(1);
  }

  // Read the file contents into a stringstream
  std::stringstream buffer;
  buffer << file.rdbuf();

  // Close the file
  file.close();

  // Extract the string from the stringstream
  std::string file_contents = buffer.str();

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
