#include <iostream>

auto main() -> int {
  auto t = int{};
  std::cin >> t;

  while (t--) {
    auto x = int{};
    auto y = int{};
    auto k = int{};

    std::cin >> x >> y >> k;

    auto currx = int{0};
    auto curry = int{0};

    auto res = int{0};
    auto turn = int{0};
    while (currx != x || curry != y) {
      if (turn % 2 == 0) {
        if (currx + k <= x) {
          currx += k;
        } else if (currx + k > x) {
          currx += (x - currx);
        } else {
        }
      } else {
        if (curry + k <= y) {
          curry += k;
        } else if (curry + k > y) {
          curry += (y - curry);
        } else {
        }
      }
      ++turn;
      ++res;
    }
    std::cout << res << std::endl;
  }
}
