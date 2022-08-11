import codewars_test as test


def multiply(a: int, b: int) -> int:
    return a * b


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(multiply(2, 1), 2)
