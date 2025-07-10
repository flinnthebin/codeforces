#include <bits/stdc++.h>

auto main() -> int {
  std::string n;
  std::cin >> n;
  std::vector<char> nv(n.begin(), n.end());

  bool flag = true;
  for (auto& i : nv) {
    if (i != '4' && i != '7') {
      flag = false;
    }
  }

  int x = std::stoi(n);
  bool aflag = false;
  std::vector<int> lucky = {4, 7, 44, 47, 74, 77};
  for (auto& l : lucky) {
    if (x % l == 0) {
      aflag = true;
    }
  }

  if (flag || aflag) {
    std::cout << "YES\n";
  } else {
    std::cout << "NO\n";
  }
}
