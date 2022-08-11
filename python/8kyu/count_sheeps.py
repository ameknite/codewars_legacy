def count_sheeps(sheep: list[bool]) -> int:
    return sum(1 for x in sheep if x)


# def count_sheeps(sheep: list[bool]) -> int:
#     return sheep.count(True)
