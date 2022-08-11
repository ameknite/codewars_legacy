import codewars_test as test


def are_you_playing_banjo(name: str) -> str:
    if name.startswith('R') or name.startswith('r'):
        return f"{name} plays banjo"
    return f"{name} does not play banjo"


@test.describe("Fixed Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(are_you_playing_banjo(
            "martin"), "martin does not play banjo")
        test.assert_equals(are_you_playing_banjo("Rikke"), "Rikke plays banjo")
        test.assert_equals(are_you_playing_banjo(
            "bravo"), "bravo does not play banjo")
        test.assert_equals(are_you_playing_banjo("rolf"), "rolf plays banjo")
