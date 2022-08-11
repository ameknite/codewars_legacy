from dis import dis
import codewars_test as test


def pillars(num_pill: int, dist: int, width: int) -> int:
    if num_pill == 1:
        return 0
    return (num_pill - 1) * dist * 100 + (num_pill - 2) * width


test.describe("Basic Tests")
test.assert_equals(pillars(1, 10, 10), 0)
test.assert_equals(pillars(2, 20, 25), 2000)
test.assert_equals(pillars(11, 15, 30), 15270)
