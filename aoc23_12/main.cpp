// let's say I have a space of N units.
// I need to fit in 1 block of x unit size.
// The number of position I can place the block in would be N - (x - 1).
// Okay, now instead of 1 block of x unit size, I have two blocks, one of x unit
// size, and one of y unit size.
// The number of possible positions would then be:
// f(N, x, y) = Sum(y, N - x, i - (y - 1))
// Now, what if instead we have three blocks of size x, y, z?
// f(N, x, y, z) = Sum(y + z, N - x, Sum(z, i - y, j - (z - 1)))
// => Sum(y + z, N - x, f(i, y, z))
//
// I think I'll do the brute force approach for now

#include <cassert>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

void parseInput(std::string fileName,
                std::vector<std::vector<bool>> &allConditions,
                std::vector<std::vector<int>> &allUnknownIdxs,
                std::vector<std::vector<int>> &allGroups);
int f(std::vector<std::vector<bool>> &allConditions,
      std::vector<std::vector<int>> &allUnknownIdxs,
      std::vector<std::vector<int>> &allGroups);
int calculateValidCombinations(std::vector<bool> &conditions,
                               std::vector<int> &unknownIdxs,
                               std::vector<int> &groups);
void setUnknownConditions(std::vector<bool> &conditions,
                          std::vector<int> &unknownIdxs, int combination);
bool checkConditions(std::vector<bool> &conditions, std::vector<int> &groups);
int exponential(int base, int exponent);
// helper
template <typename T> void printVector(std::vector<T> &v);

int main(int argc, char *argv[]) {
  {
    std::vector<std::vector<bool>> allConditions;
    std::vector<std::vector<int>> allUnknownIdxs;
    std::vector<std::vector<int>> allGroups;
    parseInput("test", allConditions, allUnknownIdxs, allGroups);
    assert(f(allConditions, allUnknownIdxs, allGroups) == 21);
  }
  {
    std::vector<std::vector<bool>> allConditions;
    std::vector<std::vector<int>> allUnknownIdxs;
    std::vector<std::vector<int>> allGroups;
    parseInput("input", allConditions, allUnknownIdxs, allGroups);
    std::cout << "Result: " << f(allConditions, allUnknownIdxs, allGroups)
              << std::endl;
  }
  return 0;
}

int f(std::vector<std::vector<bool>> &allConditions,
      std::vector<std::vector<int>> &allUnknownIdxs,
      std::vector<std::vector<int>> &allGroups) {
  int result = 0;
  for (int i = 0; i < allConditions.size(); i++) {
    result += calculateValidCombinations(allConditions[i], allUnknownIdxs[i],
                                         allGroups[i]);
  }
  return result;
}

int calculateValidCombinations(std::vector<bool> &conditions,
                               std::vector<int> &unknownIdxs,
                               std::vector<int> &groups) {
  auto result = 0;
  auto numberOfCombinations = exponential(2, unknownIdxs.size());
  for (int i = 0; i < numberOfCombinations; i++) {
    setUnknownConditions(conditions, unknownIdxs, i);
    if (checkConditions(conditions, groups)) {
      result += 1;
    }
  }
  return result;
}

void setUnknownConditions(std::vector<bool> &conditions,
                          std::vector<int> &unknownIdxs, int combination) {
  for (int i = 0; i < unknownIdxs.size(); i++) {
    conditions[unknownIdxs[i]] = (combination >> i) & 1;
  }
}

bool checkConditions(std::vector<bool> &conditions, std::vector<int> &groups) {
  auto groupIdx = 0;
  auto count = 0;
  auto hasCounted = false;
  // Check counts for each group
  for (auto condition : conditions) {
    if (!condition) {
      if (groupIdx == groups.size()) {
        return false;
      }
      count += 1;
      hasCounted = true;
    } else if (hasCounted) {
      if (count != groups[groupIdx]) {
        return false;
      }
      count = 0;
      groupIdx += 1;
      hasCounted = false;
    }
  }
  // Check if last count is correct
  if (hasCounted) {
    if (count != groups[groupIdx]) {
      return false;
    }
    groupIdx += 1;
  }
  // Check if all groups have been accounted
  if (groupIdx != groups.size()) {
    return false;
  }
  return true;
}

int exponential(int base, int exponent) {
  if (exponent == 0) {
    return 1;
  }
  return base * exponential(base, exponent - 1);
}

void parseInput(std::string fileName,
                std::vector<std::vector<bool>> &allConditions,
                std::vector<std::vector<int>> &allUnknownIdxs,
                std::vector<std::vector<int>> &allGroups) {
  std::ifstream ifs(fileName);
  while (!ifs.eof()) {
    // Get line
    std::string line;
    std::getline(ifs, line);
    if (line.length() == 0) {
      break;
    }
    // Get space index
    auto spaceIdx = line.find_first_of(" ");
    // Parse condition line
    auto conditionLine = line.substr(0, spaceIdx);
    std::vector<bool> conditions;
    std::vector<int> unknownIdxs;
    for (int i = 0; i < conditionLine.length(); i++) {
      auto c = conditionLine[i];
      if (c == '.' || c == '?') {
        conditions.push_back(true);
      } else {
        conditions.push_back(false);
      }
      if (c == '?') {
        unknownIdxs.push_back(i);
      }
    }
    allConditions.push_back(std::move(conditions));
    allUnknownIdxs.push_back(std::move(unknownIdxs));
    // Parse group line
    auto groupLine = line.substr(spaceIdx + 1);
    // Get comma seperated int out of groupLine
    std::vector<int> groups;
    auto commaIdx = groupLine.find_first_of(",");
    while (commaIdx != std::string::npos) {
      groups.push_back(std::stoi(groupLine.substr(0, commaIdx)));
      groupLine = groupLine.substr(commaIdx + 1);
      commaIdx = groupLine.find_first_of(",");
    }
    // Get last int out of groupLine
    groups.push_back(std::stoi(groupLine));
    allGroups.push_back(std::move(groups));
  }
}

// vector print function
template <typename T> void printVector(std::vector<T> &v) {
  for (auto e : v) {
    std::cout << e << " ";
  }
  std::cout << std::endl;
}
