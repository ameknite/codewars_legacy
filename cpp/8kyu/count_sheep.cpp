#include <vector>

using namespace std;

int count_sheep(vector<bool> arr)
{
    auto total{0};
    for (auto &&i : arr)
    {
        total += i ? 1 : 0;
    }
    return total;
}

// #include <algorithm>
// #include <vector>

// int count_sheep(std::vector<bool> v)
// {
//     return std::count(v.cbegin(), v.cend(), true);
// }
