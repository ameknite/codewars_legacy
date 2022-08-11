#include <vector>

int grow(std::vector<int> nums)
{
    int total{1};
    for (auto &&i : nums)
    {
        total *= i;
    }
    return total;
}
