import codewars_test as test


def correct(s: str) -> str:
    return s.translate(s.maketrans('501', 'SOI'))


@test.describe("Example tests")
def _():
    @test.it("Example tests")
    def _():
        test.assert_equals(correct("L0ND0N"), "LONDON")
        test.assert_equals(correct("DUBL1N"), "DUBLIN")
        test.assert_equals(correct("51NGAP0RE"), "SINGAPORE")
        test.assert_equals(correct("BUDAPE5T"), "BUDAPEST")
        test.assert_equals(correct("PAR15"), "PARIS")
