import codewars_test as test


def make_upper_case(s: str) -> str:
    return s.upper()


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(make_upper_case("hello"), "HELLO")
