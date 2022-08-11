#include <string>

std::string no_space(const std::string &x)
{
    std::string new_str = "";
    for (auto &&i : x)
    {
        if (i == ' ')
        {
            continue;
        }
        new_str += i;
    }
    return new_str;
}

// std::string no_space(std::string x)
// {
//     x.erase(std::remove(x.begin(), x.end(), ' '), x.end());
//     return x;
// }
