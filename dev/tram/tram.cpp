#include <bits/stdc++.h>

auto main() -> int {
  int n;
  std::cin >> n;
  int x; int y;
  std::vector<std::pair<int, int>> stops;

  while (std::cin >> x >> y) {
    stops.push_back(std::make_pair(x, y));
  }

  int count = 0;
  int max = 0;

  for (auto& v : stops) {
    count -= v.first;
    count += v.second;
    if (count > max) {
      max = count;
    }
  }
  
  std::cout << max << "\n";
}
