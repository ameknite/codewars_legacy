int summation(int num)
{
    auto total{0};
    for (size_t i = 1; i < num; i++)
    {
        total += i;
    }
    return total;
}

// int summation(int num)
// {
//     return num * (num + 1) / 2;
// }
