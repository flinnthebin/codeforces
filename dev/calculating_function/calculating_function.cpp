#include <bits/stdc++.h>

auto main() -> int {
  long long n;
  std::cin >> n;

  if (n % 2 == 0) {
    std::cout << n / 2 << '\n';
  } else {
    std::cout << -(n / 2 + 1) << '\n';
  } 
}
