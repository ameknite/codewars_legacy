#include <vector>

std::vector<int> countPositivesSumNegatives(std::vector<int> input)
{
    if (input.empty())
    {
        return {};
    }
    int count_positive{}, sum_negative{};
    for (auto &&x : input)
    {
        x > 0 ? count_positive++ : sum_negative += x;
    }
    return {count_positive, sum_negative};
}
