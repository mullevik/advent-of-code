from aoc_python.utils import load_stripped_lines


def game_outcome(opponent: str, me: str) -> str:
    if (
        (opponent == "A" and me == "X")
        or (opponent == "B" and me == "Y")
        or (opponent == "C" and me == "Z")
    ):
        return "draw"

    if (
        (opponent == "A" and me == "Y")
        or (opponent == "B" and me == "Z")
        or (opponent == "C" and me == "X")
    ):
        return "win"
    return "loss"


def get_my_strat(opponent: str, end: str) -> str:
    if opponent == "A" and end == "X":  # rock
        return "Z"
    if opponent == "A" and end == "Y":
        return "X"
    if opponent == "A" and end == "Z":
        return "Y"
    if opponent == "B" and end == "X":  # paper
        return "X"
    if opponent == "B" and end == "Y":
        return "Y"
    if opponent == "B" and end == "Z":
        return "Z"
    if opponent == "C" and end == "X":  # scissors
        return "Y"
    if opponent == "C" and end == "Y":
        return "Z"
    if opponent == "C" and end == "Z":
        return "X"


def score_game(game: str) -> int:
    opponent, end = game.split(" ")
    me = get_my_strat(opponent, end)

    _outcome = game_outcome(opponent, me)

    my_score = None
    if me == "X":
        my_score = 1
    elif me == "Y":
        my_score = 2
    elif me == "Z":
        my_score = 3
    else:
        raise ValueError("my score")

    outcome = None
    if _outcome == "win":
        outcome = 6
    elif _outcome == "draw":
        outcome = 3
    elif _outcome == "loss":
        outcome = 0
    else:
        raise ValueError("outcome")

    return my_score + outcome


if __name__ == "__main__":
    games = load_stripped_lines("inputs/02_1")

    game_scores = [score_game(x) for x in games]

    print(sum(game_scores))
