#include <cstdlib>
#include <iostream>
#include <limits>
#include <numeric>
#include <vector>

auto main() -> int {
  auto t = int{};
  std::cin >> t;

  while (t--) {
    auto n = int{};
    auto k = int{};

    std::cin >> n >> k;

    auto v = std::vector<int>(n, 0);
    std::iota(v.begin(), v.end(), k);

    auto min = std::numeric_limits<int>::max();
    for (auto i = 1; i <= n; ++i) {
      auto sum = std::accumulate(v.begin(), v.begin() + i, 0);
      auto sub = std::accumulate(v.begin() + i, v.end(), 0);

      auto res = abs(sum - sub);
      if (res < min) {
        min = res;
      }
    }
    std::cout << min << std::endl;
  }
}
