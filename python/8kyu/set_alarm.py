import codewars_test as test


def set_alarm(employed: bool, vacation: bool) -> bool:
    return employed and not vacation


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(set_alarm(True, True), False,
                           "Fails when input is True, True")
        test.assert_equals(set_alarm(False, True), False,
                           "Fails when input is False, True")
        test.assert_equals(set_alarm(False, False), False,
                           "Fails when input is False, False")
        test.assert_equals(set_alarm(True, False), True,
                           "Fails when input is True, False")

