pub fn input_generator(input: &str) -> Vec<&str> {
    input.trim().split('\n').collect()
}

pub fn solve_part1(input: Vec<&str>) -> usize {
    let mut elves = input
        .iter()
        .map(|cals| cals.trim().parse::<usize>().ok())
        .collect::<Vec<Option<usize>>>()
        .split(|x| x.is_none())
        .map(|x| x.iter().map(|val| val.unwrap()).sum())
        .collect::<Vec<usize>>();
    elves.sort();
    let elves_len = elves.len();
    dbg!(*&elves[elves_len - 3..].iter().sum::<usize>())
    // let mut cals = 0;
    // let mut biggest_cals = 0;
    // for val in input {
    //     if val.trim().len() != 0 {
    //         cals += val.trim().parse::<usize>().unwrap();
    //     } else {
    //         if cals > biggest_cals {
    //             biggest_cals = cals;
    //         }
    //         cals = 0;
    //     }
    // }
    // if cals > biggest_cals {
    //     biggest_cals = cals;
    // }
    // biggest_cals
}

fn main() {
    const INPUT: &'static str = include_str!("../../input/2022/day1.txt");
    // dbg!(input_generator(INPUT));
    let test_input = "20
    30

    20

    40";
    dbg!(solve_part1(input_generator(test_input)));
    let test_input2 = "20
    30

    20

    10

    70
    80";
    dbg!(solve_part1(input_generator(test_input2)));
    dbg!(solve_part1(input_generator(INPUT)));
}
