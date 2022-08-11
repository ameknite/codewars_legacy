import codewars_test as test


def merge_arrays(arr1: list[int], arr2: list[int]) -> list[int]:
    return sorted(set(arr1 + arr2))


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(merge_arrays([1, 2, 3, 4], [5, 6, 7, 8]), [
                           1, 2, 3, 4, 5, 6, 7, 8])
        test.assert_equals(merge_arrays([1, 3, 5, 7, 9], [10, 8, 6, 4, 2]), [
                           1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        test.assert_equals(merge_arrays([1, 3, 5, 7, 9, 11, 12], [
                           1, 2, 3, 4, 5, 10, 12]), [1, 2, 3, 4, 5, 7, 9, 10, 11, 12])
