import math
import codewars_test as test


def nearest_sq(n: int) -> int:
    next_sqrt = abs(math.ceil(math.sqrt(n))**2)
    previous_sqrt = abs(math.floor(math.sqrt(n))**2)
    return next_sqrt if abs(next_sqrt - n) < abs(previous_sqrt - n) else previous_sqrt


# def nearest_sq(n):
#     return round(n ** 0.5) ** 2


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(nearest_sq(1), 1)
        test.assert_equals(nearest_sq(2), 1)
        test.assert_equals(nearest_sq(10), 9)
        test.assert_equals(nearest_sq(111), 121)
        test.assert_equals(nearest_sq(9999), 10000)
