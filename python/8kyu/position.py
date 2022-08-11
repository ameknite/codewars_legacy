import codewars_test as test


def position(alphabet: str) -> str:
    return f"Position of alphabet: {ord(alphabet) - ord('a') + 1}"


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():

        tests = [
            # [input, expected]
            ["a", "Position of alphabet: 1"],
            ["z", "Position of alphabet: 26"],
            ["e", "Position of alphabet: 5"],
        ]

        for inp, exp in tests:
            test.assert_equals(position(inp), exp)
