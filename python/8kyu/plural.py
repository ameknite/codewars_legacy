import codewars_test as test


def plural(n: int | float) -> bool:
    return n != 1


@test.describe("Sample tests")
def _():
    @test.it("Tests")
    def __():
        test.assert_equals(plural(0), True)
        test.assert_equals(plural(1), False)
        test.assert_equals(plural(100), True)
