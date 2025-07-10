#include <bits/stdc++.h>

auto main() -> int {
  int n; std::vector<int> v;
  std::cin >> n;
  int x;
  while (std::cin >> x) {
    v.push_back(x);
  }

  int prev = 0; int count = 0;
  for (auto& m : v) {
    if (prev != m) {
      count++;
    }
    prev = m;
  }
  std::cout << count << "\n";
}
