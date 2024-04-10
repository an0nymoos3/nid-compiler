/*
 * This file handles the first step of assembling. It breaks down a .ass file
 * into its smaller "tokens" or chars.
 */
#include <string>
#include <vector>

enum TokenType { Operation, Mode, Register, Constant, Comment, EOL, Separator };

struct Token {
  std::string value;
  TokenType token_type;
};

struct Line {
  std::vector<Token> line_tokens;
  std::string line_content;
  int line_number;
};

std::vector<Line> tokenize(std::vector<Line> &file_content);

std::vector<Token> tokenize_line(std::string line_content);

void export_tokens(std::vector<Line> &lines);

void print_token(std::vector<Token> &tokens);

bool is_letter(char c);

bool is_number(char c);

std::string build_word(std::string src_code, int start_pos);

std::string build_num(std::string src_code, int start_pos);
