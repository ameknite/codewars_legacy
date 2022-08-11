import codewars_test as test


def odd_count(n: int) -> int:
    return n // 2


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(odd_count(15), 7)
        test.assert_equals(odd_count(15023), 7511)
