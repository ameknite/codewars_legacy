int past(int h, int m, int s)
{
    return (h * 3600 + m * 60 + s) * 1000;
}

// #include <chrono>

// int past(int h, int m, int s)
// {
//     std::chrono::milliseconds millis{0};
//     millis += std::chrono::hours{h};
//     millis += std::chrono::minutes{m};
//     millis += std::chrono::seconds{s};
//     return millis.count();
// }
