import codewars_test as test


def get_sum(a: int, b: int) -> int:
    return sum(range(min(a, b), max(a, b) + 1))

# def get_sum(a, b):
#     return (a + b) * (abs(a - b) + 1) // 2

# def get_sum(a, b):
#     if a > b:
#         a, b = b, a
#     return sum(range(a, b+1))


@test.describe('Sum of numbers')
def desc1():
    @test.it('Sample tests')
    def it1():
        test.assert_equals(get_sum(0, 1), 1)
        test.assert_equals(get_sum(0, -1), -1)
