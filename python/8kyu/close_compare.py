import codewars_test as test


def close_compare(a: float, b: float, margin: float = 0) -> int:
    if margin >= abs(a-b):
        return 0
    return 1 if a > b else -1


test.it("No margin")
test.assert_equals(close_compare(4, 5), -1)
test.assert_equals(close_compare(5, 5), 0)
test.assert_equals(close_compare(6, 5), 1)

test.it("With margin of 3")
test.assert_equals(close_compare(2, 5, 3), 0)
test.assert_equals(close_compare(5, 5, 3), 0)
test.assert_equals(close_compare(8, 5, 3), 0)
test.assert_equals(close_compare(8.1, 5, 3), 1)
test.assert_equals(close_compare(1.99, 5, 3), -1)
