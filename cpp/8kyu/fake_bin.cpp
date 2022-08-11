#include <string>

std::string fakeBin(std::string str)
{
    std::string result{};
    for (auto &&x : str)
    {
        x < '5' ? result += '0' : result += '1';
    }
    return result;
}
