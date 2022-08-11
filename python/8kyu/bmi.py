from re import L
import codewars_test as test


def bmi(weight: int, height: float) -> str:
    bmi = weight / height**2
    match bmi:
        case bmi if bmi <= 18.5:
            return "Underweight"
        case bmi if bmi <= 25:
            return "Normal"
        case bmi if bmi <= 30:
            return "Overweight"
        case _:
            return "Obese"


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(bmi(50, 1.80), "Underweight")
        test.assert_equals(bmi(80, 1.80), "Normal")
        test.assert_equals(bmi(90, 1.80), "Overweight")
        test.assert_equals(bmi(110, 1.80), "Obese")
        test.assert_equals(bmi(50, 1.50), "Normal")
