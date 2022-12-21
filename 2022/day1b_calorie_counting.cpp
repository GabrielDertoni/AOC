#include <bits/stdc++.h>


int main() {
    std::string line;
    std::vector<size_t> top_calories(3, 0);
    while (std::getline(std::cin, line)) {
        size_t total = 0;
        do {
            if (line.empty()) break;
            total += std::stoi(line);
        } while (std::getline(std::cin, line));

        if (top_calories[0] < total) {
            std::ranges::pop_heap(top_calories, std::greater<size_t>{});
            top_calories.back() = total;
            std::ranges::push_heap(top_calories, std::greater<size_t>{});
        }
    }
    std::cout << std::accumulate(top_calories.begin(), top_calories.end(), 0) << std::endl;
    return 0;
}

