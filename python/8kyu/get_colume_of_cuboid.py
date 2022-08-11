import codewars_test as test


def get_volume_of_cuboid(length: float, width: float, height: float) -> float:
    return length * width * height


@test.describe('Testing...')
def _():
    @test.it('Sample tests')
    def _():
        test.assert_equals(get_volume_of_cuboid(1, 2, 2), 4)
        test.assert_equals(get_volume_of_cuboid(6.3, 2, 5), 63)
