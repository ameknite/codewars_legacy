import codewars_test as test


def grow(arr: list[int]) -> int:
    product = 1
    for x in arr:
        product *= x
    return product

# import math
# def grow(arr):
#     return math.prod(arr)


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        tests = [
            [6, [1, 2, 3]],
            [16, [4, 1, 1, 1, 4]],
            [64, [2, 2, 2, 2, 2, 2]],
        ]

        for exp, inp in tests:
            test.assert_equals(grow(inp), exp)
