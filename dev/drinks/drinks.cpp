#include <bits/stdc++.h>

auto main() -> int {
  int n; int x; std::vector<int> d;
  std::cin >> n;
  while (std::cin >> x) {
    d.push_back(x);
  }
  double vol = 0;
  for (auto& i : d) {
    vol += i;
  }

  double res = ((vol / (n * 100)) * 100);
  std::cout << res << '\n';
}
