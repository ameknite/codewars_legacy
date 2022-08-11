#include <string>
using namespace std;

string reverseString(string str)
{
  std::reverse(str.begin(), str.end());
  return str;
}
