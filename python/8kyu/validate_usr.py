import codewars_test as test
import re


def validate_usr(username: str) -> bool:
    return bool(re.match('^[a-z0-9_]{4,16}$', username))

# def validate_usr(username: str) -> bool:
#     return bool(re.fullmatch('[a-z0-9_]{4,16}', username))


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(validate_usr('asddsa'), True)
        test.assert_equals(validate_usr('a'), False)
        test.assert_equals(validate_usr('Hass'), False)
        test.assert_equals(validate_usr(
            'Hasd_12assssssasasasasasaasasasasas'), False)
        test.assert_equals(validate_usr(''), False)
        test.assert_equals(validate_usr('____'), True)
        test.assert_equals(validate_usr('012'), False)
        test.assert_equals(validate_usr('p1pp1'), True)
        test.assert_equals(validate_usr('asd43 34'), False)
        test.assert_equals(validate_usr('asd43_34'), True)
