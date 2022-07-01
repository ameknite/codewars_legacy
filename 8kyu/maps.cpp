#include <vector>
#include <algorithm>

std::vector<int> maps(const std::vector<int> &values)
{
    std::vector<int> result{};
    std::transform(values.cbegin(), values.cend(), std::back_inserter(result), [](int x)
                   { return x * 2; });
    return result;
}
