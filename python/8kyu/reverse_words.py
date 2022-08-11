import codewars_test as test


def reverse_words(s: str) -> str:
    return ' '.join(s.split()[::-1])


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it("Basic Tests")
    def basic_tests():
        test.assert_equals(reverse_words("hello world!"), "world! hello")
