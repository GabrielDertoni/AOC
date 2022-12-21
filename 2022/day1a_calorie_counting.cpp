#include <bits/stdc++.h>


int main() {
    std::string line;
    size_t top_calories = 0;
    while (std::cin >> line) {
        size_t total = 0;
        do {
            if (line.empty()) break;
            total += std::stoi(line);
        } while (std::cin >> line);

        if (top_calories < total)
            top_calories = total;
    }
    return 0;
}
