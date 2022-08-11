import codewars_test as test


def square_or_square_root(arr: list[int]) -> list[int]:
    return [x**0.5 if (x**0.5).is_integer() else x**2 for x in arr]


# def square_or_square_root(arr):
#     result = []
#     for x in arr:
#         root = x**0.5
#         if root.is_integer():
#             result.append(root)
#         else:
#             result.append(x*x)
#     return result


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        tests = [
            [[4, 3, 9, 7, 2, 1], [2, 9, 3, 49, 4, 1]],
            [[100, 101, 5, 5, 1, 1], [10, 10201, 25, 25, 1, 1]],
            [[1, 2, 3, 4, 5, 6], [1, 4, 9, 2, 25, 36]],
        ]

        for inp, exp in tests:
            test.assert_equals(square_or_square_root(inp), exp)
