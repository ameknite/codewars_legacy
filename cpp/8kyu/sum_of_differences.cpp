#include <vector>
#include <numeric>
#include <cstddef>
#include <iostream>

int sumOfDifferences(const std::vector<int> &arr)
{
    if (arr.size() < 1)
    {
        return 0;
    }
    std::vector<int> new_arr{arr};
    std::sort(new_arr.begin(), new_arr.end(), std::greater{});
    int total{};
    for (size_t i = 0; i < new_arr.size() - 1; i++)
    {
        total += new_arr[i] - new_arr[i + 1];
    }
    return total;
}

// #include <vector>
// #include <algorithm>
// int sumOfDifferences(const std::vector<int> &arr)
// {
//     if (arr.empty())
//         return 0;
//     auto p = std::minmax_element(arr.cbegin(), arr.cend());
//     return *p.second - *p.first;
// }
