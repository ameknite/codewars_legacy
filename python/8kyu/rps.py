import codewars_test as test


def rps(p1: str, p2: str) -> str:
    match p1, p2:
        case "rock", "scissors":
            return "Player 1 won!"
        case "scissors", "paper":
            return "Player 1 won!"
        case "paper", "rock":
            return "Player 1 won!"
        case "scissors", "rock":
            return "Player 2 won!"
        case "paper", "scissors":
            return "Player 2 won!"
        case "rock", "paper":
            return "Player 2 won!"
        case  _:
            return "Draw!"


# def rps(p1, p2):
#     beats = {'rock': 'scissors', 'scissors': 'paper', 'paper': 'rock'}
#     if beats[p1] == p2:
#         return "Player 1 won!"
#     if beats[p2] == p1:
#         return "Player 2 won!"
#     return "Draw!"


@test.describe("rock paper scissors")
def basic_tests():
    @test.it("Player 1 wins")
    def player_1():
        test.assert_equals(rps('rock', 'scissors'), "Player 1 won!")

    @test.it("Player 1 wins")
    def player_1():
        test.assert_equals(rps('scissors', 'rock'), "Player 2 won!")

    @test.it("Draw")
    def draw():
        test.assert_equals(rps('rock', 'rock'), 'Draw!')
