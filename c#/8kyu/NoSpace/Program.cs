using System;
using System.Linq;

namespace Solution
{
    public static class SpacesRemover
    {
        public static string NoSpace(string input)
        {
            return new string(input.Where(c => !Char.IsWhiteSpace(c)).ToArray());
        }
    }
}
