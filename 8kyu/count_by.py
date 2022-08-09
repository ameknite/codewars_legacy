def count_by(x: int, n: int) -> list[int]:
    """
    Return a sequence of numbers counting by `x` `n` times.
    """
    return [x for x in range(x, x * n + 1, x)]
