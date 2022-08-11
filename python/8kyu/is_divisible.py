import codewars_test as test


def is_divisible(n: int, x: int, y: int) -> bool:
    return n % x == 0 and n % y == 0


@test.describe("Sample Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(is_divisible(3, 2, 2), False)
        test.assert_equals(is_divisible(3, 3, 4), False)
        test.assert_equals(is_divisible(12, 3, 4), True)
        test.assert_equals(is_divisible(8, 3, 4), False)
