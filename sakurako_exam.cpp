#include <iostream>

auto main() -> int {
  auto t = int{};
  std::cin >> t;

  while (t--) {
    auto a = int{};
    auto b = int{};

    std::cin >> a >> b;

    auto sum = a + 2 * b;

    if (sum % 2 == 0 && (a >= (sum / 2) % 2)) {
      std::cout << "Yes" << std::endl;
    } else {
      std::cout << "No" << std::endl;
    }
  }
}
