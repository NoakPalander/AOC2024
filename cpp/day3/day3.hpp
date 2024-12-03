#include <filesystem>
#include <fstream>
#include <regex>
#include <ranges>
#include <algorithm>

namespace day3 {
    namespace fs = std::filesystem;

    inline std::string read_file(fs::path const& path) {
        std::ifstream reader(path);
        return {std::istreambuf_iterator(reader), std::istreambuf_iterator<char>{}};
    }

    inline int part_one(std::string const& data) {
        std::regex const pattern(R"(mul\((\d{1,3}),(\d{1,3})\))");
        std::sregex_iterator const begin(data.begin(), data.end(), pattern);
        std::sregex_iterator const end;

        auto muls = std::ranges::subrange(begin, end) | std::views::transform([](auto&& match) {
            return std::stoi(match[1].str()) * std::stoi(match[2].str());
        });

        return std::ranges::fold_left(muls, 0, std::plus<>{});
    }

    inline int part_two(std::string const& data) {
        std::regex const pattern(R"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))");
        std::sregex_iterator const begin(data.begin(), data.end(), pattern);
        std::sregex_iterator const end;

        bool ignore = false;
        auto muls = std::ranges::subrange(begin, end) | std::views::transform([&ignore](auto&& match) {
            if (match[0].str() == "don't()") {
                ignore = true;
                return 0;
            }
            else if (match[0].str() == "do()") {
                ignore = false;
                return 0;
            }

            return ignore ? 0 : std::stoi(match[1].str()) * std::stoi(match[2].str());
        });

        return std::ranges::fold_left(muls, 0, std::plus<>{});
    }
}
