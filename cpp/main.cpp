#include "day3/day3.hpp"
#include <filesystem>
#include <print>

int main() {
    auto path = std::filesystem::current_path().parent_path() / "day3" / "input.txt";
    auto data = day3::read_file(path);

    std::println("Part one: {}", day3::part_one(data));
    std::println("Part two: {}", day3::part_two(data));
}
