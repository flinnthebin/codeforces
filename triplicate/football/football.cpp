#include <bits/stdc++.h>

auto main() -> int {
  std::string num;
  std::cin >> num;

  int zeroes = 0; int ones = 0; bool res = false;

  for (int i = 0; i < num.length(); ++i) {
    if (num[i] == '0') {
      zeroes++;
      ones = 0;
    } else {
      ones++;
      zeroes = 0;
    }
    if (zeroes >= 7 || ones >= 7) {
      res = true;
    }
  }

  if (res) {
    std::cout << "YES\n";
  } else {
    std::cout << "NO\n";
  }
}
