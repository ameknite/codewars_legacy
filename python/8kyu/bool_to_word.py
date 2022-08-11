import codewars_test as test


def bool_to_word(boolean: bool) -> str:
    return 'Yes' if boolean else 'No'


@test.describe("Fixed Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(bool_to_word(True), 'Yes')
        test.assert_equals(bool_to_word(False), 'No')
