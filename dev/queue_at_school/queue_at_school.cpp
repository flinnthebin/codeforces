#include <bits/stdc++.h>

auto main() -> int {
  int n; int t; std::string q;
  std::cin >> n >> t >> q;
  std::vector<char> queue(q.begin(), q.end());

  for (int _ = 0; _ < t; ++_) {
    int x = 0;
    while (x < n - 1) {
      if (queue[x] == 'B' && queue[x+1] == 'G') {
        std::iter_swap(queue.begin() + x, queue.begin() + x + 1);
        x++;
      }
      x++;
    }
  }
  std::string result = std::string(queue.begin(), queue.end());
  std::cout << result << "\n";
  }
