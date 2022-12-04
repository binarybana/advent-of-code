use itertools::Itertools;

pub fn input_generator(input: &str) -> Vec<&str> {
    input.trim().split("\n\n").collect()
}

pub fn solve_part2(input: Vec<&str>) -> usize {
    input
        .iter()
        .map(|elf| -> usize {
            elf.lines()
                .map(|cals| cals.trim().parse::<usize>().unwrap())
                .sum()
        })
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum()
}

fn main() {
    const INPUT: &'static str = include_str!("../../input/2022/day1.txt");
    // dbg!(input_generator(INPUT));
    let test_input = "20
    30

    20

    40";
    dbg!(solve_part2(input_generator(test_input)));
    let test_input2 = "20
    30

    20

    10

    70
    80";
    dbg!(solve_part2(input_generator(test_input2)));
    dbg!(solve_part2(input_generator(INPUT)));
}
