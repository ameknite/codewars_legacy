#include <vector>

std::vector<int> digitize(unsigned long n)
{
    if (n == 0)
    {
        return std::vector<int>{0};
    }
    std::vector<int> vec;
    while (n != 0)
    {
        vec.push_back(n % 10);
        n /= 10;
    }
    return vec;
}
