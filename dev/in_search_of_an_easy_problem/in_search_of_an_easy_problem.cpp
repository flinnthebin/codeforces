#include <bits/stdc++.h>

auto main() -> int {
  int n; std::vector<int> v;
  std::cin >> n;

  int x;
  while (std::cin >> x) {
    v.push_back(x);
  }
  
  int tar = 1;
  auto it = std::find(v.begin(), v.end(), tar);
  if (it != v.end()) {
    std::cout << "HARD\n";
  } else {
    std::cout << "EASY\n";
  }
}
