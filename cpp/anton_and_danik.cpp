#include <bits/stdc++.h>

auto main() -> int {
  int n; std::string s;
  std::cin >> n >> s;
  auto anton = std::ranges::count(s, 'A');
  auto danik = std::ranges::count(s, 'D');
  if (anton == danik) {
    std::cout << "Friendship\n";
  } else if (anton > danik) {
    std::cout << "Anton\n";
  } else {
    std::cout << "Danik\n";
  }
}
