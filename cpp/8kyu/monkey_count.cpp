#include <vector>

std::vector<int> MonkeyCount(int n)
{
    std::vector<int> monkeys;
    monkeys.reserve(n);
    for (size_t i = 1; i <= n; i++)
    {
        monkeys.push_back(i);
    }
    return monkeys;
}
