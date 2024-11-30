#include <bits/stdc++.h>

auto main() -> int {
  std::ios::sync_with_stdio(0);
  std::cin.tie(0);

  auto n = int{}; // number of test cases
  auto a = int{}; auto b = int{}; // two sacred numbers
  auto m = int{}; // m >= max(a, b) (Least Common Multiple)
                  // m mod a = m mod b (Greatest Common Divisor)
  std::cin >> n;

  for (auto x = 0; x < n; ++x) {
    std::cin >> a >> b;
    auto gcd = std::gcd(a, b);
    auto m = (a / gcd) * b;
    std::cout << m << "\n";
  }
}
