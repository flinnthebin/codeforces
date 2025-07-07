#include <bits/stdc++.h>

auto main() -> int {
  std::string first; std::string second;
  std::cin >> first >> second;
  std::reverse(second.begin(), second.end());
  if (first == second) {
    std::cout << "YES\n";
  } else {
    std::cout << "NO\n";
  }
}
