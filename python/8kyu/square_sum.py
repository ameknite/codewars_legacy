import codewars_test as test


def square_sum(numbers: list) -> int:
    return sum(x**2 for x in numbers)


@test.describe("Fixed Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(square_sum([1, 2]), 5)
        test.assert_equals(square_sum([0, 3, 4, 5]), 50)
        test.assert_equals(square_sum([]), 0)
        test.assert_equals(square_sum([-1, -2]), 5)
        test.assert_equals(square_sum([-1, 0, 1]), 2)
