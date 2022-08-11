import codewars_test as test


def past(h: int, m: int, s: int) -> int:
    return (h * 3600 + m * 60 + s) * 1000


@test.describe("Fixed Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(past(0, 1, 1), 61000)
        test.assert_equals(past(1, 1, 1), 3661000)
        test.assert_equals(past(0, 0, 0), 0)
        test.assert_equals(past(1, 0, 1), 3601000)
        test.assert_equals(past(1, 0, 0), 3600000)
