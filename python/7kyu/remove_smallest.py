import codewars_test as test
from numpy.random import randint


def remove_smallest(numbers: list[int]) -> list[int]:
    if len(numbers) == 0:
        return []

    smallest_position = tuple((0, numbers[0]))
    for x in enumerate(numbers):
        if x[1] < smallest_position[1]:
            smallest_position = x

    result = []
    for x in enumerate(numbers):
        if x == smallest_position:
            continue
        result.append(x[1])
    return result


# def remove_smallest(numbers):
#     a = numbers[:]
#     if a:
#         a.remove(min(a))
#     return a


test.describe("remove_smallest")

test.it("works for the examples")
test.assert_equals(remove_smallest([1, 2, 3, 4, 5]), [
                   2, 3, 4, 5], "Wrong result for [1, 2, 3, 4, 5]")
test.assert_equals(remove_smallest([5, 3, 2, 1, 4]), [
                   5, 3, 2, 4], "Wrong result for [5, 3, 2, 1, 4]")
test.assert_equals(remove_smallest([1, 2, 3, 1, 1]), [
                   2, 3, 1, 1], "Wrong result for [1, 2, 3, 1, 1]")
test.assert_equals(remove_smallest([]), [], "Wrong result for []")


def randlist():
    return list(randint(400, size=randint(1, 10)))


test.it("returns [] if list has only one element")
for i in range(10):
    x = randint(1, 400)
    test.assert_equals(remove_smallest(
        [x]), [], "Wrong result for [{}]".format(x))

test.it("returns a list that misses only one element")
for i in range(10):
    arr = randlist()
    test.assert_equals(len(remove_smallest(arr[:])), len(
        arr) - 1, "Wrong sized result for {}".format(arr))
