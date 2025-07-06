#include <bits/stdc++.h>

auto main() -> int {
  int n; int m;
  std::cin >> n;
  std::vector<int> vectors;
  while (std::cin >> m) {
    vectors.push_back(m);
  }

  int x = 0; int y = 1; int z = 2;
  int xcnt = 0; int ycnt = 0; int zcnt = 0;

  for (auto i = 0; i < n; ++i) {
    xcnt += vectors[x];
    ycnt += vectors[y];
    zcnt += vectors[z];

    x += 3; y += 3; z += 3;
  }

  if (xcnt == 0 && ycnt == 0 && zcnt == 0) {
    std::cout << "YES" << "\n";
  } else {
    std::cout << "NO" << "\n";
  }
}
