#include <bits/stdc++.h>

auto main() -> int {
  int n; int m; int a; int b;
  std::cin >> n >> m >> a >> b;

  auto cost_a = n * a;
  int full_tickets = n / m;
  auto leftover = n % m;
  auto cost_b = full_tickets * b + leftover * a;
  int cost_c = ((n + m - 1) / m) * b;

  std::cout << std::min({cost_a, cost_b, cost_c}) << "\n";
}
