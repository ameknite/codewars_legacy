import codewars_test as test


def greet(name: str) -> str:
    if name == "Johnny":
        return "Hello, my love!"
    return f"Hello, {name}!"


test.describe("Jenny's greeting function")
test.it("should greet some people normally")
test.assert_equals(greet("James"), "Hello, James!")
test.assert_equals(greet("Jane"), "Hello, Jane!")
test.assert_equals(greet("Jim"), "Hello, Jim!")

test.it("should greet Johnny a little bit more special")
test.assert_equals(greet("Johnny"), "Hello, my love!")
