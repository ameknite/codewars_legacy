#include <vector>
#include <algorithm>
using namespace std;

int min(vector<int> list)
{
    return *std::min_element(list.cbegin(), list.cend());
}

int max(vector<int> list)
{
    return *std::max_element(list.cbegin(), list.cend());
}
