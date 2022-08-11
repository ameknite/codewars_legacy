import codewars_test as test


def index(array: list[int], n: int) -> int:
    try:
        return array[n] ** n
    except:
        return -1


# def index(array, n):
#     try:
#         return array[n]**n
#     except IndexError:
#         return -1


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(index([1, 2, 3, 4], 2), 9)
        test.assert_equals(index([1, 3, 10, 100], 3), 1000000)
