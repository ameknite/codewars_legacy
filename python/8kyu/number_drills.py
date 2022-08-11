import codewars_test as test


def is_divisible(wall_length: int, pixel_size: int) -> bool:
    return wall_length % pixel_size == 0


test.assert_equals(is_divisible(4050, 27), True)
test.assert_equals(is_divisible(4066, 27), False)
test.assert_equals(is_divisible(10000, 20), True)
test.assert_equals(is_divisible(10005, 20), False)
