#include <algorithm>
#include <array>

unsigned short int expressionsMatter(unsigned short int a, unsigned short int b, unsigned short int c)
{
    std::array arr{a * b * c, a + b + c, (a + b) * c, a * (b + c)};
    return *std::max_element(arr.begin(), arr.end());
}

// #include <algorithm>

// unsigned short expressionsMatter(unsigned short a, unsigned short b, unsigned short c)
// {
//     return std::max({a + b + c, a * (b + c), (a + b) * c, a * b * c});
// }
