pub type Round = (Play, Play);
#[derive(Debug)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

pub fn input_generator(input: &str) -> Vec<Round> {
    fn match_other(hand: &str) -> Play {
        match hand {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("{hand}"),
        }
    }

    fn match_mine(hand: &str) -> Play {
        match hand {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("{hand}"),
        }
    }
    input
        .trim()
        .split('\n')
        .map(|round| {
            if let [other, mine] = &round.split(' ').collect::<Vec<&str>>()[..] {
                (match_other(other), match_mine(mine))
            } else {
                panic!("{round}")
            }
        })
        .collect()
}

pub fn solve_part1(input: Vec<Round>) -> usize {
    use Play::*;
    input
        .iter()
        .map(|round| match round {
            (Rock, Rock) => 1 + 3,
            (Rock, Paper) => 2 + 6,
            (Rock, Scissors) => 3 + 0,
            (Paper, Rock) => 1 + 0,
            (Paper, Paper) => 2 + 3,
            (Paper, Scissors) => 3 + 6,
            (Scissors, Rock) => 1 + 6,
            (Scissors, Paper) => 2 + 0,
            (Scissors, Scissors) => 3 + 3,
        })
        .sum()
}

fn main() {
    const INPUT: &'static str = include_str!("../../input/2022/day2.txt");
    // dbg!(input_generator(INPUT));
    let test_input = "A X
B X
C X";
    dbg!(input_generator(test_input));
    dbg!(solve_part1(input_generator(test_input)));
    dbg!(solve_part1(input_generator(INPUT)));
}
