import codewars_test as test


def lovefunc(flower1: int, flower2: int) -> bool:
    return (flower1 + flower2) % 2 == 1


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(lovefunc(1, 4), True)
        test.assert_equals(lovefunc(2, 2), False)
        test.assert_equals(lovefunc(0, 1), True)
        test.assert_equals(lovefunc(0, 0), False)
