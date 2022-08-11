import codewars_test as test


def correct_tail(body: str, tail: str) -> bool:
    return body.endswith(tail)


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(correct_tail("Fox", "x"), True)
        test.assert_equals(correct_tail("Rhino", "o"), True)
        test.assert_equals(correct_tail("Meerkat", "t"), True)
        test.assert_equals(correct_tail("Emu", "t"), False)
        test.assert_equals(correct_tail("Badger", "s"), False)
        test.assert_equals(correct_tail("Giraffe", "d"), False)
