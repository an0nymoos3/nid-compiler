/*
 * This file handles the output from the assembler. It takes care or whether you
 * want to export to a binary blob or print the PM to terminal to copy, paste
 * into a VHDL file.
 */

#include "exporter.hpp"
#include <fstream>

/**
 * Writes assembled code into an output file.
 */
void write_to_file(std::vector<AssembeledLine> &binary_content,
                   std::string outname) {
  std::ofstream output;
  output.open(outname);

  for (const AssembeledLine binline : binary_content) {
    output << binline.line_content;
  }

  output.close();
}
