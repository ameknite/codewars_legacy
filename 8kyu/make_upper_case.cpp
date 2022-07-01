#include <string>
#include <algorithm>

std::string makeUpperCase(const std::string &input_str)
{
    std::string str{};
    str.reserve(input_str.size());
    std::transform(input_str.cbegin(), input_str.cend(), std::back_inserter(str), [](char c)
                   { return std::toupper(c); });
    return str;
}
