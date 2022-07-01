#include <vector>

std::vector<int> invert(std::vector<int> values)
{
    std::vector<int> vec{};
    for (auto &&x : values)
    {
        vec.push_back(-x);
    }
    return vec;
}

// #include <vector>
// #include <algorithm>

// std::vector<int> invert(std::vector<int> values)
// {
//     transform(values.begin(), values.end(), values.begin(), std::negate<int>());
//     ;
//     return values;
// }
