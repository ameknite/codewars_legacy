#include <vector>
std::vector<int> countBy(int x, int n)
{
    std::vector<int> count;
    count.reserve(n);
    for (size_t i = 1; i <= n; i++)
    {
        count.push_back(i * x);
    }
    return count;
}
