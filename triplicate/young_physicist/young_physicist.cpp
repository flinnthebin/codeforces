#include <bits/stdc++.h>

auto main() -> int {
  int n; int m;
  std::cin >> n;

  std::vector<std::array<int, 3>> vectors(n);

  for (int i = 0; i < n; ++i) {
    std::cin >> vectors[i][0] >> vectors[i][1] >> vectors[i][2];
  }

  int xcnt = 0, ycnt = 0, zcnt = 0;

  for (const auto& vec : vectors) {
    xcnt += vec[0];
    ycnt += vec[1];
    zcnt += vec[2];
  }

  if (xcnt == 0 && ycnt == 0 && zcnt == 0) {
    std::cout << "YES\n";
  } else {
    std::cout << "NO\n";
  }
}
