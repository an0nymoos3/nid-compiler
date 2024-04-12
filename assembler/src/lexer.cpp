/*
 * This file handles the first step of assembling. It breaks down a .ass file
 * into its smaller "tokens" or chars.
 */
#include "lexer.hpp"
#include <iostream>
#include <vector>

std::vector<Line> tokenize(std::vector<Line> &file_content) {
  std::vector<Line> lines;

  for (int i = 0; i < file_content.size(); i++) {
    std::vector<Token> token_queue;
    Line current_line = file_content[i];
    token_queue = tokenize_line(current_line.line_content);
    token_queue = check_token_line(token_queue);
    current_line.line_tokens = token_queue;
    lines.push_back(current_line);
  }

  return lines;
}

std::vector<Token> tokenize_line(std::string line_content) {
  std::vector<Token> token_queue;
  std::vector<char> src_code;

  // Push all the chars to src_code
  for (char c : line_content) {
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
    else if (is_letter(current_char)) {
      std::string word = build_word(line_content, j);
      token = {word, Operation};
      j += word.size() - 1; // Skip past the rest of the built word
      token_queue.push_back(token);
    }

    /*
     * Checks what type the number represents
     */
    else if (is_number(current_char)) {
      std::string word = build_num(line_content, j);
      j += word.size() - 1; // Skip past the rest of the built word

      if (word.size() == 2) {
        token = {word, Mode};
      } else if (word.size() == 4) {
        token = {word, Register};
      } else {
        token = {word, Constant};
      }
      token_queue.push_back(token);
    } else {
      if (current_char != ' ') {
        std::cout << "Unknown char!" << std::endl;
      }
    }
  }

  return token_queue;
}

std::vector<Token> check_token_line(std::vector<Token> token_line) {
  if (token_line.size() == 0) {
    return token_line;
  }

  // Check if line is a typical "Operation, Mode, Register, Const" line
  if (token_line[0].token_type == Operation) {
    if (token_line[2].token_type != Mode) {
      Token token = {",", Separator};
      token_line.insert(token_line.begin() + 1, token);
      Token token2 = {"00", Mode};
      token_line.insert(token_line.begin() + 2, token2);
    }

    if (token_line[4].token_type != Register) {
      Token token = {",", Separator};
      token_line.insert(token_line.begin() + 3, token);
      Token token2 = {"0000", Register};
      token_line.insert(token_line.begin() + 4, token2);
    }

    if (token_line[6].token_type != Constant) {
      Token token = {",", Separator};
      token_line.insert(token_line.begin() + 5, token);
      Token token2 = {"0000000000000000", Constant};
      token_line.insert(token_line.begin() + 6, token2);
    }
  }

  return token_line;
}

bool is_letter(char c) { return std::isalpha(c); }

bool is_number(char c) { return std::isdigit(c); }

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
