import codewars_test as test


def rental_car_cost(d: int) -> int:
    total = d * 40
    if d >= 7:
        total -= 50
    elif d >= 3:
        total -= 20
    return total


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(rental_car_cost(1), 40)
        test.assert_equals(rental_car_cost(4), 140)
        test.assert_equals(rental_car_cost(7), 230)
        test.assert_equals(rental_car_cost(8), 270)
