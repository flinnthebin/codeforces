#include <bits/stdc++.h>

auto main() -> int {
  int n; int h;
  std::cin >> n >> h;

  int x; std::vector<int> vec;
  while (std::cin >> x) {
    vec.push_back(x);
  }

  int count = 0;
  for (auto& v : vec) {
    if (v <= h) {
      count++;
    } else {
      count+=2;
    }
  }

  std::cout << count << "\n";
}
