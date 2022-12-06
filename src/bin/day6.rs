use itertools::Itertools;
// use regex::Regex;

pub fn solve_p1(input: &str, window: usize) -> usize {
    input.chars().collect::<Vec<char>>()[..]
        .windows(window)
        // .inspect(|x| println!("{:?}", x))
        .map(|w| w.into_iter().unique().count())
        .position(|x| x == window)
        .unwrap()
        + window
}

fn main() {
    // bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
    // nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
    // nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
    // zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
    dbg!(solve_p1("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    dbg!(solve_p1("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    dbg!(solve_p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    dbg!(solve_p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    const INPUT: &'static str = include_str!("../../input/2022/day6.txt");
    // dbg!(solve_p1(INPUT, 4));

    // mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
    // bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
    // nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
    // nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
    // zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26
    dbg!(solve_p1("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 19);
    dbg!(solve_p1("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    dbg!(solve_p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    dbg!(solve_p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    dbg!(solve_p1(INPUT, 14));
}
