#include <iostream>

auto main() -> int {
  auto t = int{};
  std::cin >> t;

  while (t--) {
    auto n = int{};
    auto s = std::string{};

    std::cin >> n >> s;

    if (s[n / 2] == '0') {
      std::cout << "Yes" << std::endl;
    } else {
      std::cout << "No" << std::endl;
    }
  }
}
