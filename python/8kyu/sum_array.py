import codewars_test as test


def sum_array(a: list[int | float]) -> float:
    return sum(a)


@test.describe("Testing sum array")
def tests():
    @test.it("Fixed tests")
    def fixed_tests():
        test.assert_equals(sum_array([]), 0)
        test.assert_equals(sum_array([1, 2, 3]), 6)
        test.assert_equals(sum_array([1.1, 2.2, 3.3]), 6.6)
        test.assert_equals(sum_array([4, 5, 6]), 15)
        test.assert_equals(sum_array(range(101)), 5050)
