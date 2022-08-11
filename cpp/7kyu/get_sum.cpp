#include <math.h>

int get_sum(int a, int b)
{
    int total{};
    for (int i = std::min(a, b); i <= std::max(a, b); i++)
    {
        total += i;
    }
    return total;
}

// int get_sum(int a, int b)
// {
//     return (a + b) * (std::abs(a - b) + 1) / 2;
// }
