import codewars_test as test


def hex_to_dec(s: str) -> int:
    return int(s, 16)


@test.describe("Fixead Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(hex_to_dec("1"), 1)
        test.assert_equals(hex_to_dec("a"), 10)
        test.assert_equals(hex_to_dec("10"), 16)
