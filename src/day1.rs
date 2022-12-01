const INPUT: &'static str = include_str!("../input/2022/day1.txt");

pub fn input_generator(input: &str) -> Vec<&str> {
    input.trim().split('\n').collect()
}

pub fn solve_part1(input: Vec<&str>) -> usize {
    let mut cals = 0;
    let mut biggest_cals = 0;
    for val in input {
        if val.trim().len() != 0 {
            cals += val.trim().parse::<usize>().unwrap();
        } else {
            if cals > biggest_cals {
                biggest_cals = cals;
            }
            cals = 0;
        }
    }
    if cals > biggest_cals {
        biggest_cals = cals;
    }
    biggest_cals
}

#[test]
fn test() {
    // dbg!(input_generator(INPUT));
    let test_input = "20
    30

    40";
    dbg!(solve_part1(input_generator(test_input)));
    let test_input2 = "20
    30

    70
    80";
    dbg!(solve_part1(input_generator(test_input2)));
    dbg!(solve_part1(input_generator(INPUT)));
    // assert_eq!(calc_children(8, 7), 1);
}
