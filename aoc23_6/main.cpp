#include <cassert>
#include <cmath>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

void parseInput(std::string fileName, std::vector<int> &times,
                std::vector<int> &distances);
int calculateMargin(int time, int distance);
int f(const std::vector<int> &times, const std::vector<int> &distances);

int main(int argc, char *argv[]) {
  {
    std::vector<int> times;
    std::vector<int> distances;
    parseInput("test", times, distances);
    assert(f(times, distances) == 288);
  }
  {
    std::vector<int> times;
    std::vector<int> distances;
    parseInput("input", times, distances);
    std::cout << "Result: " << f(times, distances) << std::endl;
  }

  return 0;
}

int f(const std::vector<int> &times, const std::vector<int> &distances) {
  int N = times.size();
  int result = 1;
  for (int i = 0; i < N; i++) {
    result *= calculateMargin(times[i], distances[i]);
  }
  return result;
}

void parseInput(std::string fileName, std::vector<int> &times,
                std::vector<int> &distances) {
  std::ifstream ifs(fileName);
  std::string line;
  std::regex colon(".*:\\s*(\\w.*\\w).*");
  std::regex number("\\d+");
  std::smatch sm;

  // Times
  std::getline(ifs, line);
  // Skip header
  std::regex_search(line, sm, colon);
  auto numbersLine = sm[1].str();
  // Get numbers
  while (std::regex_search(numbersLine, sm, number)) {
    int number = std::stoi(sm.str());
    times.push_back(number);
    numbersLine = sm.suffix();
  }

  // Distances
  std::getline(ifs, line);
  // Skip header
  std::regex_search(line, sm, colon);
  numbersLine = sm[1].str();
  // Get numbers
  while (std::regex_search(numbersLine, sm, number)) {
    int number = std::stoi(sm.str());
    distances.push_back(number);
    numbersLine = sm.suffix();
  }
}

// (T - t) * t > D -> t*t - T*t + D < 0 -> (t - a)(t - b) < 0,
// where a, b = (T +- sqrt(T*T - 4*D))/2
//
// The inequality is satisfied when t < ceil(a) and t > floor(b)
//
// Consequently, the range is determined as: ceil(a) - floor(b) - 1
//
// For example, in the first race, a, b = (7 +- 3.6) / 2 = 5.3, 1.7
//
// Therefore, the margin is 6 - 1 - 1 = 4
//
// In the second rance, a, b = (15 +- 8.06) / 2 = 11.5, 3.5
//
// Therefore, the margin is 12 - 3 - 1 = 8
int calculateMargin(int time, int distance) {
  auto D = std::sqrt(time * time - 4 * distance);
  int a = std::ceil((time + D) / 2.0);
  int b = std::floor((time - D) / 2.0);
  return a - b - 1;
}
