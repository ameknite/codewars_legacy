#include <vector>
#include <cmath>

int index(const std::vector<int> &vector, int n)
{
    try
    {
        return std::pow(vector.at(n), n);
    }
    catch (const std::out_of_range &e)
    {
        return -1;
    }
}
