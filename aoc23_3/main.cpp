#include <cassert>
#include <cctype>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <utility>

struct Part {
  int number;
  std::pair<int, int> position;
  int length;
};

void parse_input(std::string file_name,
                 std::vector<std::vector<char>> &input_matrix,
                 std::vector<Part> &parts);
int f(const std::vector<std::vector<char>> &input_matrix,
      const std::vector<Part> &parts);
bool check_valid_part(const std::vector<std::vector<char>> &input_matrix,
                      const Part &part);

int main(int argc, char *argv[]) {
  {
    std::vector<std::vector<char>> input_matrix;
    std::vector<Part> parts;
    parse_input("test", input_matrix, parts);
    assert(f(input_matrix, parts) == 4361);
  }
  std::vector<std::vector<char>> input_matrix;
  std::vector<Part> parts;
  parse_input("input", input_matrix, parts);
  std::cout << "Result: " << f(input_matrix, parts) << std::endl;
  return 0;
}

int f(const std::vector<std::vector<char>> &input_matrix,
      const std::vector<Part> &parts) {
  int result = 0;
  for (int i = 0; i < parts.size(); i++) {
    if (check_valid_part(input_matrix, parts[i])) {
      result += parts[i].number;
    }
  }
  return result;
}

bool check_valid_part(const std::vector<std::vector<char>> &input_matrix,
                      const Part &part) {
  int nrows = input_matrix.size();
  int ncols = input_matrix[0].size();
  int row = part.position.first;
  int start = part.position.second;
  int end = start + part.length - 1;
  if (start - 1 >= 0 && input_matrix[row][start - 1] != '.' &&
      !isdigit(input_matrix[row][start - 1])) {
    return true;
  }
  if (end + 1 < ncols && input_matrix[row][end + 1] != '.' &&
      !isdigit(input_matrix[row][end + 1])) {
    return true;
  }
  if (row - 1 >= 0) {
    for (int i = start - 1; i <= end + 1; i++) {
      if (i >= 0 && i < ncols && input_matrix[row - 1][i] != '.' &&
          !isdigit(input_matrix[row - 1][i])) {
        return true;
      }
    }
  }
  if (row + 1 < nrows) {
    for (int i = start - 1; i <= end + 1; i++) {
      if (i >= 0 && i < ncols && input_matrix[row + 1][i] != '.' &&
          !isdigit(input_matrix[row + 1][i])) {
        return true;
      }
    }
  }
  return false;
}

void parse_input(std::string file_name,
                 std::vector<std::vector<char>> &input_matrix,
                 std::vector<Part> &parts) {
  std::ifstream f{file_name};
  int row = 0;
  while (!f.fail()) {
    // get line
    std::string line;
    f >> line;
    if (f.fail())
      break;

    // turn input to character array
    std::vector<char> input_char_row;
    for (int i = 0; i < line.length(); i++) {
      input_char_row.push_back(line[i]);
    }
    input_matrix.push_back(input_char_row);

    // match part numbers using regex
    std::regex number_regex{"([\\d]+)"};
    int pos = 0;
    for (std::smatch matches; std::regex_search(line, matches, number_regex);) {
      auto number = std::stoi(matches.str());
      auto length = (int)matches.str().length();
      auto position = std::make_pair(row, pos + matches.position());
      parts.push_back(Part{number, position, length});
      line = matches.suffix();
      pos += matches.position() + length;
    }

    row += 1;
  }
}
