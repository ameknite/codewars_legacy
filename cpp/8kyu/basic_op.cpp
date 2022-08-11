#include <stdexcept>

int basicOp(char op, int val1, int val2)
{
    switch (op)
    {
    case '+':
        return val1 + val2;
    case '-':
        return val1 - val2;
    case '*':
        return val1 * val2;
    case '/':
        return val1 / val2;
    default:
        throw std::invalid_argument("Invalid operator");
    }
}
