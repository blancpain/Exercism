#include <string>

namespace log_line {
std::string message(std::string line) {
  std::string subStr = line.substr(line.find(":") + 2);
  return subStr;
}

std::string log_level(std::string line) {
  int start = line.find("[") + 1;
  int end = line.find("]") + 1;
  std::string subStr = line.substr(start, end);
  return subStr;
}

std::string reformat(std::string line) {
  std::string output = message(line) + log_level(line);
  return output;
}
} // namespace log_line
