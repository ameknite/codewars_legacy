import codewars_test as test


def hero(bullets: int, dragons: int) -> bool:
    return bullets >= dragons * 2


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(hero(10, 5), True)
        test.assert_equals(hero(7, 4), False)
        test.assert_equals(hero(4, 5), False)
        test.assert_equals(hero(100, 40), True)
        test.assert_equals(hero(1500, 751), False)
        test.assert_equals(hero(0, 1), False)
