import codewars_test as test


def name_shuffler(s: str) -> str:
    return ' '.join(s.split()[::-1])


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(name_shuffler('john McClane'), 'McClane john')
        test.assert_equals(name_shuffler('Mary jeggins'), 'jeggins Mary')
        test.assert_equals(name_shuffler('tom jerry'), 'jerry tom')
