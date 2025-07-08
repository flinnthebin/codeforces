#include <bits/stdc++.h>

bool hasUniqueDigits(int year) {
  std::string s = std::to_string(year);
  std::set<char> digits(s.begin(), s.end());
  return digits.size() == s.size();
}

auto main() -> int {
  int y;
  std::cin >> y;

  int x = y + 1;
  while (!hasUniqueDigits(x)) {
    ++x;
  }

  std::cout << x << "\n";
  return 0;
}
