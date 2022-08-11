#include <vector>

int square_sum(const std::vector<int> &numbers)
{
    int sum = 0;
    for (auto &&x : numbers)
    {
        sum += x * x;
    }
    return sum;
}

// #include <numeric>
// #include <vector>

// int square_sum(const std::vector<int> &numbers)
// {
//     return std::accumulate(numbers.begin(), numbers.end(), 0, [](int a, int b)
//                            { return a + b * b; });
// }

// #include <vector>
// #include <numeric>

// int square_sum(const std::vector<int> &v)
// {
//     int x = std::inner_product(v.begin(), v.end(), v.begin(), 0);
//     return x;
// }
