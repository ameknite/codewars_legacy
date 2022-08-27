using System;
using System.Linq;

public class Kata
{
    public static int PositiveSum(int[] arr)
    {
        return arr.Sum(x => (x > 0 ? x : 0));
    }
}
