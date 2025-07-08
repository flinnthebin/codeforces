#include <bits/stdc++.h>

auto main() -> int {
  int n; int k;
  std::cin >> n >> k;
  for (auto i = 0; i < k ; ++i) {
    if (n % 10 == 0) {
      n /= 10;
    } else {
      n -= 1;
    }
  }
  std::cout << n << "\n";
}
