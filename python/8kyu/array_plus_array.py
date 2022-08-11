import codewars_test as test


def array_plus_array(arr1: list[int], arr2: list[int]) -> int:
    arr1.extend(arr2)
    return sum(arr1)


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(array_plus_array([1, 2, 3], [4, 5, 6]), 21)
        test.assert_equals(array_plus_array([-1, -2, -3], [-4, -5, -6]), -21)
        test.assert_equals(array_plus_array([0, 0, 0], [4, 5, 6]), 15)
        test.assert_equals(array_plus_array(
            [100, 200, 300], [400, 500, 600]), 2100)
