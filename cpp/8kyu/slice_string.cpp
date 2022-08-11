#include <string>

using namespace std;

string sliceString(string str)
{
    std::string result = "";
    for (int i = 1; i <= str.length() - 2; i++)
    {
        result += str[i];
    }
    return result;
}

// #include <string>
// using namespace std;

// string sliceString(string str)
// {
//     return str.substr(1, str.size() - 2);
// }

// #include <string>
// using namespace std;

// string sliceString(string str)
// {
//     str.erase(str.begin());
//     str.erase(str.length() - 1);
//     return str;
// }
