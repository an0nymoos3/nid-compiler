/*
 * This file handles the first step of assembling. It breaks down a .ass file
 * into its smaller "tokens" or chars.
 */
#include "lexer.hpp"
#include "utils/errors.hpp"
#include <algorithm>
#include <bitset>
#include <iostream>
#include <sstream>
#include <vector>

std::vector<Line> tokenize(std::vector<Line> &file_content,
                           bool &assembly_failed) {
  std::vector<Line> lines;

  for (int i = 0; i < file_content.size(); i++) {
    std::vector<Token> token_queue;
    Line current_line = file_content[i];
    token_queue = tokenize_line(current_line, assembly_failed);
    token_queue = check_token_line(token_queue, current_line.line_number);
    current_line.line_tokens = token_queue;
    lines.push_back(current_line);
  }

  return lines;
}

std::vector<Token> tokenize_line(Line &line, bool &assembly_failed) {
  std::vector<Token> token_queue;
  std::vector<char> src_code;
  // Vector for tokens {Operation, Mode, Register} added to line
  std::vector<bool> broken_structure = {false, false, false};

  // For Jmp* tokens
  std::vector<std::string> jmp_ops = {"jmp", "jsr", "ret", "beq", "bne", "bpl",
                                      "bmi", "bge", "blt", "byk", "bnk"};
  std::string prev_op = "nop";

  // Push all the chars to src_code
  for (char c : line.line_content) {
    src_code.push_back(c);
  }

  for (int j = 0; j < src_code.size(); j++) {
    char current_char = src_code[j];
    Token token;

    /*
     * Check for token separator
     */
    if (current_char == ',') {
      token = {",", Separator};
      token_queue.push_back(token);
    }

    /*
     * Check for comments
     */
    else if (current_char == ';') {
      token = {";", Comment};
      token_queue.push_back(token);
      return token_queue; // Return early when line ends
    }

    /*
     * Checks for end of line
     */
    else if (current_char == '\n') {
      token = {"|n", EOL};
      token_queue.push_back(token);
      return token_queue; // Return early when line ends
    }

    /*
     * Checks for assembly operation
     */
    else if (is_letter(current_char) && !is_register(line.line_content, j) &&
             !is_mode(line.line_content, j) && !broken_structure[1] &&
             !broken_structure[2] &&
             build_word(line.line_content, j).size() > 1) {
      std::string word = build_word(line.line_content, j);
      token = {word, Operation};
      j += word.size() - 1; // Skip past the rest of the built word
      token_queue.push_back(token);
      broken_structure[0] = true;
      prev_op = word;
    }

    /*
     * Checks for adress mode
     */
    else if (is_mode(line.line_content, j) && !broken_structure[2] &&
             ((int)line.line_content[++j] - 48) <= 3) {
      std::string word = build_num(line.line_content, j);
      j += word.size() - 1; // Skip past the rest of the built word
      word = decimal_to_binary(word);
      if (word.size() >= 1) {
        word = word.substr(14, 2);
      }
      token = {word, Mode};
      token_queue.push_back(token);
      broken_structure[1] = true;
    }

    /*
     * Checks for register index
     */
    else if (is_register(line.line_content, j) &&
             std::stoi(build_num(line.line_content, ++j), nullptr, 10) <= 15) {
      std::string word = build_num(line.line_content, j);
      j += word.size() - 1; // Skip past the rest of the built word
      word = decimal_to_binary(word);
      if (word.size() >= 1) {
        word = word.substr(12, 4);
      }
      token = {word, Register};
      token_queue.push_back(token);
      broken_structure[2] = true;
    }

    /*
     * Checks for constant
     */
    else if (is_number(current_char)) {
      std::string word = build_num(line.line_content, j);
      j += word.size() - 1; // Skip past the rest of the built word
      word = decimal_to_binary(word);
      token = {word, Constant};
      token_queue.push_back(token);
    }

    /*
     * Checks for jump OP
     */
    else if (current_char == '#' && std::find(jmp_ops.begin(), jmp_ops.end(),
                                              prev_op) != jmp_ops.end()) {
      j++;
      std::string word = build_word(line.line_content, j);
      token = {word, JmpOP};
      j += word.size() - 1; // Skip past the rest of the built word
      token_queue.push_back(token);
    }

    /*
     * Checks for jump point
     */
    else if (current_char == '#') {
      j++;
      std::string word = build_word(line.line_content, j);
      token = {word, JmpPoint};
      j += word.size() - 1; // Skip past the rest of the built word
      token_queue.push_back(token);
    }

    /*
     * Base case for either blank space and wrong inputs
     */
    else {
      if (current_char != ' ') {
        std::stringstream ss;
        ss << "Error: Unknown char supplied: " << current_char;
        Error err = {line.line_number, ss.str(), line.line_content};
        print_error(err);
        assembly_failed = true;
        token = {"|n", EOL};
        token_queue.push_back(token);
      }
    }
  }

  return token_queue;
}

std::vector<Token> check_token_line(std::vector<Token> token_line,
                                    int line_number) {
  if (token_line.size() == 0) {
    return token_line;
  }

  // Check if line is a typical "Operation, Mode, Register, Const" line
  if (token_line[0].token_type == Operation) {
    if (token_line.size() <= 2 || token_line[2].token_type != Mode) {
      Token token = {",", Separator};
      token_line.insert(token_line.begin() + 1, token);
      Token token2 = {"00", Mode};
      token_line.insert(token_line.begin() + 2, token2);
    }

    if (token_line.size() <= 4 || token_line[4].token_type != Register) {
      Token token = {",", Separator};
      token_line.insert(token_line.begin() + 3, token);
      Token token2 = {"0000", Register};
      token_line.insert(token_line.begin() + 4, token2);
    }

    if (token_line.size() <= 5 || token_line[5].token_type != JmpOP) {
      if (token_line.size() <= 6 || token_line[6].token_type != Constant) {
        Token token = {",", Separator};
        token_line.insert(token_line.begin() + 5, token);
        Token token2 = {"0000000000000000", Constant};
        token_line.insert(token_line.begin() + 6, token2);
      }
    }
  }

  return token_line;
}

bool is_letter(char c) { return std::isalpha(c); }

bool is_number(char c) { return std::isdigit(c); }

bool is_register(std::string src_code, int start_pos) {
  std::string word = "";
  word += src_code[start_pos];
  start_pos++;

  return word == "r" && is_number(src_code[start_pos]);
}

bool is_mode(std::string src_code, int start_pos) {
  std::string word = "";
  word += src_code[start_pos];
  start_pos++;

  return word == "m" && is_number(src_code[start_pos]);
}

std::string build_word(std::string src_code, int start_pos) {
  std::string word = "";
  word += src_code[start_pos];
  start_pos++;

  while (is_letter(src_code[start_pos])) {
    word += src_code[start_pos];
    start_pos++;
  }

  return word;
}

std::string build_num(std::string src_code, int start_pos) {
  std::string word = "";
  word += src_code[start_pos];
  start_pos++;

  while (is_number(src_code[start_pos])) {
    word += src_code[start_pos];
    start_pos++;
  }

  return word;
}

std::string decimal_to_binary(std::string decimal_string) {
  if (decimal_string.size() == 0) {
    return "";
  }

  // Convert decimal string to an integer
  int decimalNumber = std::stoi(decimal_string, nullptr, 10);

  // sizeof(int) * 8 gives the number of bits in an integer
  std::bitset<sizeof(int) * 4> bits(decimalNumber);
  std::ostringstream oss;
  oss << bits;
  return oss.str();
}

void export_tokens(std::vector<Line> &lines) {
  for (int i = 0; i < lines.size(); i++) {
    std::cout << "Printing line" << lines[i].line_number << ": " << std::endl;
    print_token(lines[i].line_tokens);
    std::cout << std::endl;
  }
}

void print_token(std::vector<Token> &tokens) {
  for (int i = 0; i < tokens.size(); i++) {
    std::cout << "Token (value, type): " << tokens[i].value << ", "
              << tokens[i].token_type << std::endl;
  }
}

std::string print_error_area(std::string line_content, int error_index) {
  std::string error_area;

  for (int i = error_index - 12; i <= error_index + 6; i++) {
    if (i >= 0 && i < line_content.size()) {
      if (i == error_index) {
        error_area += " -> ";
        error_area += line_content[i];
        error_area += " <- ";
      } else {
        error_area += line_content[i];
      }
    }
  }

  return error_area;
}
