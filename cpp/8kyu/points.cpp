#include <array>
#include <string>

int points(const std::array<std::string, 10> &games)
{
    int total{};
    for (auto &&x : games)
    {
        if (x[0] > x[2])
        {
            total += 3;
        }
        else if (x[0] == x[2])
        {
            total++;
        }
    }
    return total;
}

// #include <array>
// #include <string>
// #include <numeric>

// int points(const std::array<std::string, 10> &games)
// {
//     return std::accumulate(games.begin(), games.end(), 0, [](auto r, auto s)
//                            {
//     if(s[0] > s[2]) return r + 3;
//     if(s[0] < s[2]) return r;
//     return r + 1; });
// }
