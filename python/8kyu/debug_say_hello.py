import codewars_test as test


def say_hello(name: str) -> str:
    return f"Hello, {name}"


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(say_hello('Mr. Spock'), 'Hello, Mr. Spock')
        test.assert_equals(say_hello('Captain Kirk'), 'Hello, Captain Kirk')
        test.assert_equals(say_hello('Liutenant Uhura'),
                           'Hello, Liutenant Uhura')
        test.assert_equals(say_hello('Dr. McCoy'), 'Hello, Dr. McCoy')
        test.assert_equals(say_hello('Mr. Scott'), 'Hello, Mr. Scott')
