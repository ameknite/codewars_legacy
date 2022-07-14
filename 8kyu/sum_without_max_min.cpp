#include <vector>
#include <numeric>

int sum(std::vector<int> numbers)
{
    if (numbers.size() <= 2)
    {
        return 0;
    }
    int total = std::accumulate(numbers.begin(), numbers.end(), 0);
    auto minmax = std::minmax_element(numbers.begin(), numbers.end());
    return total - *minmax.first - *minmax.second;
}

// #include <vector>
// using namespace std;

// int sum(vector<int> numbers)
// {
//     if (numbers.size() < 2)
//         return 0;
//     int sum = 0;
//     int low = numbers[0], high = numbers[0];
//     for (int n : numbers)
//     {
//         if (n < low)
//             low = n;
//         else if (n > high)
//             high = n;
//         sum += n;
//     }
//     return sum - high - low;
// }
