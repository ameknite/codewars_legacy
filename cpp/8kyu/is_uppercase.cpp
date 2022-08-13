#include <string>

bool is_uppercase(const std::string &s)
{
    for (auto &&i : s)
    {
        if (std::islower(i))
        {
            return false;
        }
    }
    return true;
}
