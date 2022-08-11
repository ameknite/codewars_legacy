import codewars_test as test


def points(games: list[str]) -> int:
    total = 0
    for x in games:
        if x[0] > x[-1]:
            total += 3
        elif x[0] == x[-1]:
            total += 1
    return total


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(points(
            ['1:0', '2:0', '3:0', '4:0', '2:1', '3:1', '4:1', '3:2', '4:2', '4:3']), 30)
        test.assert_equals(points(
            ['1:1', '2:2', '3:3', '4:4', '2:2', '3:3', '4:4', '3:3', '4:4', '4:4']), 10)
        test.assert_equals(points(
            ['0:1', '0:2', '0:3', '0:4', '1:2', '1:3', '1:4', '2:3', '2:4', '3:4']), 0)
        test.assert_equals(points(
            ['1:0', '2:0', '3:0', '4:0', '2:1', '1:3', '1:4', '2:3', '2:4', '3:4']), 15)
        test.assert_equals(points(
            ['1:0', '2:0', '3:0', '4:4', '2:2', '3:3', '1:4', '2:3', '2:4', '3:4']), 12)
