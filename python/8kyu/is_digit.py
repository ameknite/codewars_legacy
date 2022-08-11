import re
import codewars_test as test


# def is_digit(n: str) -> bool:
#     return n != '' and n in "01234567889"


def is_digit(n: str) -> bool:
    return len(n) == 1 and bool(re.search('\d', n))


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(is_digit(""), False)
        test.assert_equals(is_digit("7"), True)
        test.assert_equals(is_digit(" "), False)
        test.assert_equals(is_digit("a"), False)
        test.assert_equals(is_digit("a5"), False)
