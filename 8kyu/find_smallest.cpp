#include <vector>
#include <algorithm>

int findSmallest(std::vector<int> list)
{
    return *std::min_element(list.begin(), list.end());
}

// #include <algorithm>
// #include <vector>

// int findSmallest(const std::vector<int> &xs)
// {
//     return *std::min_element(xs.cbegin(), xs.cend());
// }
