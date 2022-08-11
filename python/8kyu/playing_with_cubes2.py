import codewars_test as test


class Cube():
    def __init__(self, side: int = 0) -> None:
        self.__side = -side if side < 0 else side

    def get_side(self) -> int:
        """Return the side of the Cube"""
        return self.__side

    def set_side(self, new_side: int) -> None:
        """Set the value of the Cube's side."""
        self.__init__(new_side)


# class Cube():
#     def __init__(self, side: int = 0) -> None:
#         self.set_side(side)

#     def get_side(self) -> int:
#         """Return the side of the Cube"""
#         return self.__side

#     def set_side(self, new_side: int) -> None:
#         """Set the value of the Cube's side."""
#         self.__side = abs(new_side)


@test.describe('Testing Cube class')
def _():

    @test.it(f'With Cube({10})')
    def _():
        c = Cube(10)
        test.assert_equals(c.get_side(), 10)
