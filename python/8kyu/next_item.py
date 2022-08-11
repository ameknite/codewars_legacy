import codewars_test as test


def next_item(xs, item):
    if type(xs) is list or type(xs) is str:
        position = None
        try:
            position = xs.index(item)
            return xs[position+1]
        except:
            return None
    position = None
    next_item = None
    for i, x in enumerate(xs):
        if position == None and x == item:
            position = i
        if position != None and position + 1 == i:
            next_item = x
        if x > item:
            break
    return next_item


# def next_item(xs, item):
#     xs = iter(xs)
#     if item in xs:
#         return next(xs, None)


@test.describe('next_item')
def test_next_item():
    @test.it('should pass some tests')
    def tests():
        test.assert_equals(next_item([1, 2, 3, 4, 5, 6, 7, 8], 5), 6)
        test.assert_equals(next_item(['a', 'b', 'c'], 'd'), None)
        test.assert_equals(next_item(['a', 'b', 'c'], 'c'), None)
        test.assert_equals(next_item('testing', 't'), 'e')
        test.assert_equals(next_item(iter(range(1, 30000)), 12), 13)
