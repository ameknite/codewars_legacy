import codewars_test as test


def greet(name: str) -> str:
    return f"Hello, {name} how are you doing today?"


@test.describe("Fixed Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(
            greet('Ryan'), "Hello, Ryan how are you doing today?")
        test.assert_equals(greet('Shingles'),
                           "Hello, Shingles how are you doing today?")
