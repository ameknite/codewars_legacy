import codewars_test as test


def build_string(*args):
    return f"I like {', '.join(args)}!"


test.assert_equals(build_string('Cheese', 'Milk', 'Chocolate'),
                   'I like Cheese, Milk, Chocolate!', 'Return the correct String')
test.assert_equals(build_string('Cheese', 'Milk'),
                   'I like Cheese, Milk!', 'Return the correct String')
test.assert_equals(build_string('Chocolate'),
                   'I like Chocolate!', 'Return the correct String')
