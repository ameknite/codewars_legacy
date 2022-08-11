#include <string>
using namespace std;

string bonus_time(int salary, bool bonus)
{
    string res{"$"};
    if (bonus)
    {
        res += to_string(salary * 10);
    }
    else
    {
        res += to_string(salary);
    }
    return res;
}

