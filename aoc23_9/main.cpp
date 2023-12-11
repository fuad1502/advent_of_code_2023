#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

bool isAllZero(const std::vector<int> &difference);
std::vector<int> calculateDifference(std::vector<int> &difference);
int extrapolate(const std::vector<int> &history);
int extrapolate_beginning(const std::vector<int> &history);

std::vector<std::vector<int>> parseInput(std::string fileName);

int f(const std::vector<std::vector<int>> &input);
int f2(const std::vector<std::vector<int>> &input);

int main(int argc, char *argv[]) {
  {
    auto input = parseInput("test");
    assert(f(input) == 114);
  }
  {
    auto input = parseInput("input");
    std::cout << "Result: " << f(input) << std::endl;
  }
  {
    auto input = parseInput("test");
    assert(f2(input) == 2);
  }
  {
    auto input = parseInput("input");
    std::cout << "Result (part 2): " << f2(input) << std::endl;
  }
  return 0;
}

int f(const std::vector<std::vector<int>> &input) {
  int result = 0;
  for (const auto &history : input) {
    result += extrapolate(history);
  }
  return result;
}

int f2(const std::vector<std::vector<int>> &input) {
  int result = 0;
  for (const auto &history : input) {
    result += extrapolate_beginning(history);
  }
  return result;
}

int extrapolate(const std::vector<int> &history) {
  std::vector<int> lastElements;
  for (std::vector<int> difference = std::move(history); !isAllZero(difference);
       difference = calculateDifference(difference)) {
    lastElements.push_back(difference.back());
  }
  int result = 0;
  for (int i = lastElements.size() - 1; i >= 0; i--) {
    result += lastElements[i];
  }
  return result;
}

int extrapolate_beginning(const std::vector<int> &history) {
  std::vector<int> startElements;
  for (std::vector<int> difference = history; !isAllZero(difference);
       difference = calculateDifference(difference)) {
    startElements.push_back(difference.front());
  }
  int result = 0;
  for (int i = startElements.size() - 1; i >= 0; i--) {
    result = startElements[i] - result;
  }
  return result;
}

bool isAllZero(const std::vector<int> &difference) {
  for (auto e : difference) {
    if (e != 0) {
      return false;
    }
  }
  return true;
}

std::vector<int> calculateDifference(std::vector<int> &difference) {
  std::vector<int> result;
  for (int i = 1; i < difference.size(); i++) {
    result.push_back(difference[i] - difference[i - 1]);
  }
  return result;
}

std::vector<std::vector<int>> parseInput(std::string fileName) {
  std::vector<std::vector<int>> input;
  std::ifstream ifs(fileName);
  while (!ifs.eof()) {
    std::string line;
    std::getline(ifs, line);
    std::stringstream ss(line);
    std::vector<int> history;
    for (std::string word; !(ss >> word).fail();) {
      history.push_back(std::stoi(word));
    }
    input.push_back(std::move(history));
  }
  return input;
}
