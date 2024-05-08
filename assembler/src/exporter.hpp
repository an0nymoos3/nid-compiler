/*
 * This file handles the output from the assembler. It takes care or whether you
 * want to export to a binary blob or print the PM to terminal to copy, paste
 * into a VHDL file.
 */

#include "assembler.hpp"
#include <vector>

void write_to_file(std::vector<AssembeledLine> &binary_content,
                   std::string outname);
