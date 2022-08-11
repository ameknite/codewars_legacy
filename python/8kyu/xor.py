import codewars_test as test


def xor(a: bool, b: bool) -> bool:
    return a ^ b


@test.describe("Your 'xor' function/operator")
def _():

    @test.it("should work for the four basic cases described above")
    def _():
        test.assert_equals(xor(False, False), False,
                           "False xor False == False")
        test.assert_equals(xor(True, False), True, "True xor False == True")
        test.assert_equals(xor(False, True), True, "False xor True == True")
        test.assert_equals(xor(True, True), False, "True xor True == False")
