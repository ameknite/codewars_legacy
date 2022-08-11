import codewars_test as test


def boolean_to_string(b: bool) -> str:
    return str(b)


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(boolean_to_string(True), "True")
        test.assert_equals(boolean_to_string(False), "False")
