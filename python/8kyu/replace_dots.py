import codewars_test as test
import re


def replace_dots(s: str) -> str:
    return re.sub(r"\.", "-", s)


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(replace_dots(""), "")
        test.assert_equals(replace_dots("no dots"), "no dots")
        test.assert_equals(replace_dots("one.two.three"), "one-two-three")
        test.assert_equals(replace_dots("........"), "--------")
