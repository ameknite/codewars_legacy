#include <vector>

int arrayPlusArray(std::vector<int> a, std::vector<int> b)
{
    int total{};
    for (auto &&i : a)
    {
        total += i;
    }
    for (auto &&i : b)
    {
        total += i;
    }
    return total;
}
