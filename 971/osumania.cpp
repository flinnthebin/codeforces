#include <iostream>
#include <vector>

auto main() -> int {
    auto t = int{};
    std::cin >> t;

    while (t--) {
        auto n = int{};
        std::cin >> n;

        auto tar = '#';
        auto v = std::vector<int>{};

        auto row = std::string{};
        for (auto i = 0; i < n; ++i) {
            std::cin >> row;
            auto pos = row.find(tar) + 1;
            v.insert(v.begin(), pos);
        }

        for (auto const &p : v) {
            std::cout << p << " ";
        }
    }
}
