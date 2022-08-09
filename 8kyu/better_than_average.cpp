#include <vector>

bool betterThanAverage(std::vector<int> classPoints, int yourPoints)
{
    int total{};
    for (auto &&i : classPoints)
    {
        total += i;
    }
    return yourPoints > total / classPoints.size();
}
