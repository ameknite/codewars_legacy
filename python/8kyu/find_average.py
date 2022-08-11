import codewars_test as test


def find_average(numbers: list[float]) -> float:
    return sum(numbers) / len(numbers)


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(find_average([1, 2, 3]), 2)
