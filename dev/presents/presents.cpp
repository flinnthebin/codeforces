#include <bits/stdc++.h>

auto main() -> int {
  int n; int x; std::vector<int> v;
  std::cin >> n;
  while (std::cin >> x) {
    v.push_back(x);
  }

  std::vector<size_t> w;
  for (auto i = 1; i <= n; ++i) {
    std::vector<int>::iterator iter = std::find(v.begin(), v.end(), i);
    size_t idx = iter - v.begin();
    idx++;
    w.push_back(idx);
  }
  for (auto&j : w) {
    std::cout << j << ' ';
  }
  std::cout << '\n';
}
