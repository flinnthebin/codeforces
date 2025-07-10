#include <bits/stdc++.h>

auto main() -> int {
  std::string s;
  std::cin >> s;
  std::vector<char> sv(s.begin(), s.end());
  std::string res;
  for (auto& i : sv) {
    if (i == 'h' && res.empty()) {
      res.append(1, 'h');
    }
    if (i == 'e' && res == "h") {
      res.append(1, 'e');
    }
    if (i == 'l' && (res == "he" || res == "hel")) {
      res.append(1, 'l');
    }
    if (i == 'o' && res == "hell") {
      res.append(1, 'o');
    }
  }

  if (res == "hello") {
    std::cout << "YES\n";
  } else {
    std::cout << "NO\n";
  }
}

