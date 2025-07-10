#include <bits/stdc++.h>

auto main() -> int {
  int n; int p; int q; std::vector<std::pair<int, int>> rooms;
  std::cin >> n;
  while (std::cin >> p >> q) {
    auto pair = std::make_pair(p, q);
    rooms.push_back(pair);
  }
  int count = 0;
  for (auto& r : rooms) {
    if (r.second - r.first >= 2) {
      count++;
    }
  }
  std::cout << count << "\n";
}
