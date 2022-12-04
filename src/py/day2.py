def hand_points(hand: str) -> int:
    other, mine = hand.split(' ')
    if other == 'A' and mine == 'X':
        points = 1+3
    elif other == 'A' and mine == 'Y':
        points = 2+6
    elif other == 'A' and mine == 'Z':
        points = 3+0
    elif other == 'B' and mine == 'X':
        points = 1+0
    elif other == 'B' and mine == 'Y':
        points = 2+3
    elif other == 'B' and mine == 'Z':
        points = 3+6
    elif other == 'C' and mine == 'X':
        points = 1+6
    elif other == 'C' and mine == 'Y':
        points = 2+0
    elif other == 'C' and mine == 'Z':
        points = 3+3
    return points

def solve_part1(data: str) -> int:
    return sum([hand_points(hand) for hand in data.strip().split('\n')])

def test_solver():
    test_input = """A X
B X
C X""";
    print(solve_part1(test_input))
    data = open("input/2022/day2.txt").read()
    print(solve_part1(data))