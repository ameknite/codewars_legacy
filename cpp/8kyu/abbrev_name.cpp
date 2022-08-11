#include <string>
#include <sstream>

std::string abbrevName(std::string name)
{
    std::string result{};
    std::string str1{};
    std::string str2{};
    std::stringstream s{name};
    s >> str1 >> str2;
    return result + (char)toupper(str1[0]) + '.' + (char)toupper(str2[0]);
}

// #include <string>

// std::string abbrevName(std::string name)
// {
//     std::string ret;
//     ret.push_back(toupper(name[0]));
//     ret.push_back('.');
//     ret.push_back(toupper(name[name.find(' ') + 1]));
//     return ret;
// }
