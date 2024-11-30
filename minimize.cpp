#include <iostream>

auto main() -> int {
    auto t = int{};
    std::cin >> t;

    while (t--) {
        auto a = int{};
        auto b = int{};

        std::cin >> a >> b;

        for (auto c = 1; c < 10; ++c) {
            auto res = (c - a) + (b - c);
            if (res >= 0) {
                std::cout << res << std::endl;
                break;
            }
        }
    }
}
