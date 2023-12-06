#include <cassert>
#include <cmath>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

void parseInput(std::string fileName, std::vector<double> &times,
                std::vector<double> &distances, bool isSecondPart);
double calculateMargin(double time, double distance);
int f(const std::vector<double> &times, const std::vector<double> &distances);

int main(int argc, char *argv[]) {
  {
    std::vector<double> times;
    std::vector<double> distances;
    parseInput("test", times, distances, false);
    assert(f(times, distances) == 288);
  }
  {
    std::vector<double> times;
    std::vector<double> distances;
    parseInput("input", times, distances, false);
    std::cout << "Result: " << f(times, distances) << std::endl;
  }
  {
    std::vector<double> times;
    std::vector<double> distances;
    parseInput("test", times, distances, true);
    assert(f(times, distances) == 71503);
  }
  {
    std::vector<double> times;
    std::vector<double> distances;
    parseInput("input", times, distances, true);
    std::cout << "Result (part 2): " << f(times, distances) << std::endl;
  }

  return 0;
}

int f(const std::vector<double> &times, const std::vector<double> &distances) {
  int N = times.size();
  int result = 1;
  for (int i = 0; i < N; i++) {
    result *= calculateMargin(times[i], distances[i]);
  }
  return result;
}

void parseInput(std::string fileName, std::vector<double> &times,
                std::vector<double> &distances, bool isSecondPart) {
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
  if (!isSecondPart) {
    while (std::regex_search(numbersLine, sm, number)) {
      auto number = std::stod(sm.str());
      times.push_back(number);
      numbersLine = sm.suffix();
    }
  } else {
    std::string timeString = "";
    while (std::regex_search(numbersLine, sm, number)) {
      timeString.append(sm.str());
      numbersLine = sm.suffix();
    }
    times.push_back(std::stod(timeString));
  }

  // Distances
  std::getline(ifs, line);
  // Skip header
  std::regex_search(line, sm, colon);
  numbersLine = sm[1].str();
  // Get numbers
  if (!isSecondPart) {
    while (std::regex_search(numbersLine, sm, number)) {
      auto number = std::stod(sm.str());
      distances.push_back(number);
      numbersLine = sm.suffix();
    }
  } else {
    std::string distanceString = "";
    while (std::regex_search(numbersLine, sm, number)) {
      distanceString.append(sm.str());
      numbersLine = sm.suffix();
    }
    distances.push_back(std::stod(distanceString));
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
double calculateMargin(double time, double distance) {
  auto D = std::sqrt(time * time - 4.0 * distance);
  int a = std::ceil((time + D) / 2.0);
  int b = std::floor((time - D) / 2.0);
  return a - b - 1;
}
