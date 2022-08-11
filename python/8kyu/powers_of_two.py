import codewars_test as test


def powers_of_two(n: int) -> list[int]:
    return [2**x for x in range(0, n+1)]

# def powers_of_two(n):
#     return [1 << x for x in range(n + 1)]


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(powers_of_two(0), [1])
        test.assert_equals(powers_of_two(1), [1, 2])
        test.assert_equals(powers_of_two(4), [1, 2, 4, 8, 16])
