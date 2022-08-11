import codewars_test as test


def shark(pontoon_distance: float,
          shark_distance: float,
          you_speed: float,
          shark_speed: float,
          dolphin: float) -> str:
    shark_speed = shark_speed / 2 if dolphin else shark_speed
    if shark_distance / shark_speed > pontoon_distance / you_speed:
        return "Alive!"
    return "Shark Bait!"


test.assert_equals(shark(12, 50, 4, 8, True), "Alive!")
test.assert_equals(shark(7, 55, 4, 16, True), "Alive!")
test.assert_equals(shark(24, 0, 4, 8, True), "Shark Bait!")
