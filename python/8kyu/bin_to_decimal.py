import codewars_test as test


def bin_to_decimal(inp: str) -> int:
    return int(inp, 2)


# def bin_to_decimal(inp):
#     num = 0
#     inp = inp[::-1]
#     for i in range(len(inp)):
#         num += int(inp[i]) * 2 ** i
#     return num

test.assert_equals(bin_to_decimal("0"), 0)
test.assert_equals(bin_to_decimal("1"), 1)
test.assert_equals(bin_to_decimal("10"), 2)
test.assert_equals(bin_to_decimal("11"), 3)
test.assert_equals(bin_to_decimal("101010"), 42)
test.assert_equals(bin_to_decimal("1001001"), 73)
