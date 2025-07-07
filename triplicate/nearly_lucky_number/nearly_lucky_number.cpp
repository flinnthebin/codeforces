#include <bits/stdc++.h>

auto main() -> int {
  std::string num; int cnt = 0; bool res = true;
  std::cin >> num;

  for (int i = 0; i < num.length(); ++i) {
    if (num[i] == '4' || num[i] == '7') {
       cnt++;
    }
  }
  std::string check = std::to_string(cnt);
  for (int j = 0; j < check.length(); ++j) {
    if (check[j] != '4' and check[j] != '7') {
      std::cout << "NO\n";
      res = false;
      break;
    }
  }

  if (res) {
    std::cout << "YES\n";
  }
}
