import codewars_test as test


def capitalize_word(word: str) -> str:
    return word.capitalize()


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(capitalize_word('word'), 'Word')
        test.assert_equals(capitalize_word('i'), 'I')
        test.assert_equals(capitalize_word('glasswear'), 'Glasswear')
