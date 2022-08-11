import codewars_test as test


def paperwork(n: int, m: int) -> int:
    if n < 0 or m < 0:
        return 0
    return n * m


@test.describe("Fixed Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(paperwork(5, 5), 25, "Failed at Paperwork(5,5)")
        test.assert_equals(paperwork(-5, 5), 0, "Failed at Paperwork(-5,5)")
        test.assert_equals(paperwork(5, -5), 0, "Failed at Paperwork(5,-5)")
        test.assert_equals(paperwork(-5, -5), 0, "Failed at Paperwork(-5,-5)")
        test.assert_equals(paperwork(5, 0), 0, "Failed at Paperwork(5,0)")
