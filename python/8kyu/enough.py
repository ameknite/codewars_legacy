import codewars_test as test


def enough(cap: int, on: int, wait: int) -> int:
    if cap - on < wait:
        return abs(cap - on - wait)
    return 0


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(enough(10, 5, 5), 0)
        test.assert_equals(enough(100, 60, 50), 10)
        test.assert_equals(enough(20, 5, 5), 0)
