import codewars_test as test


def warn_the_sheep(queue: list[str]) -> str:
    queue.reverse()
    position = queue.index("wolf")
    match position:
        case 0:
            return "Pls go away and stop eating my sheep"
        case _:
            return f"Oi! Sheep number {position}! You are about to be eaten by a wolf!"


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(warn_the_sheep(['sheep', 'sheep', 'sheep', 'sheep', 'sheep', 'wolf',
                           'sheep', 'sheep']), 'Oi! Sheep number 2! You are about to be eaten by a wolf!')
        test.assert_equals(warn_the_sheep(['sheep', 'wolf', 'sheep', 'sheep', 'sheep',
                           'sheep', 'sheep']), 'Oi! Sheep number 5! You are about to be eaten by a wolf!')
        test.assert_equals(warn_the_sheep(['wolf', 'sheep', 'sheep', 'sheep', 'sheep',
                           'sheep', 'sheep']), 'Oi! Sheep number 6! You are about to be eaten by a wolf!')
        test.assert_equals(warn_the_sheep(
            ['sheep', 'wolf', 'sheep']), 'Oi! Sheep number 1! You are about to be eaten by a wolf!')
        test.assert_equals(warn_the_sheep(
            ['sheep', 'sheep', 'wolf']), 'Pls go away and stop eating my sheep')
