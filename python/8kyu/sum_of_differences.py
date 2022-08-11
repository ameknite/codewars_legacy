import codewars_test as test


def sum_of_differences(arr: list[int]) -> int:
    if len(arr) <= 1:
        return 0
    arr.sort(reverse=True)
    return sum(arr[i] - arr[i+1] for i in range(len(arr) - 1))


# def sum_of_differences(arr):
#     return max(arr) - min(arr) if arr else 0


@test.describe("Sample tests")
def tests():
    @test.it("Some examples")
    def tests():
        test.assert_equals(sum_of_differences([1, 2, 10]), 9)
        test.assert_equals(sum_of_differences([-3, -2, -1]), 2)
        test.assert_equals(sum_of_differences([1, 1, 1, 1, 1]), 0)
        test.assert_equals(sum_of_differences([-17, 17]), 34)
        test.assert_equals(sum_of_differences([]), 0)
        test.assert_equals(sum_of_differences([0]), 0)
        test.assert_equals(sum_of_differences([-1]), 0)
        test.assert_equals(sum_of_differences([1]), 0)
