import codewars_test as test


def is_uppercase(inp: str) -> bool:
    res = []
    for word in inp:
        if not word.isalpha():
            res.append(True)
        elif word.isupper():
            res.append(True)
        else:
            res.append(False)
    return all(res)


# def is_uppercase(inp):
#     return inp.upper() == inp


def gen_test_case(inp, res):
    test.assert_equals(is_uppercase(inp), res, inp)


test.describe("Basic Tests")

gen_test_case("c", False)
gen_test_case("C", True)
gen_test_case("hello I AM DONALD", False)
gen_test_case("HELLO I AM DONALD", True)
gen_test_case("$%&", True)
