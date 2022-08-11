#include <string>

std::string countSheep(int number)
{
    std::string res{};
    for (size_t i = 1; i <= number; i++)
    {
        res += std::to_string(i) + " sheep...";
    }
    return res;
}
