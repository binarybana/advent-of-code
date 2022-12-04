use itertools::Itertools;
use std::collections::HashSet;

fn char_to_priority(val: char) -> usize {
    if val.is_ascii_uppercase() {
        ((val as u8) - ('A' as u8) + 27) as usize
    } else {
        ((val as u8) - ('a' as u8) + 1) as usize
    }
}

pub fn solve_p1(input: &str) {
    let ans: usize = input
        .lines()
        .map(|pack| {
            let len = pack.len();
            let c1: HashSet<char> = HashSet::from_iter(pack[..len / 2].chars());
            let c2: HashSet<char> = HashSet::from_iter(pack[len / 2..].chars());
            let mut wrong = c1.intersection(&c2);
            let val = wrong.next().unwrap().clone();
            char_to_priority(val)
        })
        .sum();
    dbg!(ans);
    // .collect::<Vec<_>>();
}

pub fn solve_p2(input: &str) -> usize {
    use std::collections::hash_map::RandomState;
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|pack| {
            let h1: Option<HashSet<char, RandomState>> = pack
                .map(|elf| HashSet::from_iter(elf.chars()))
                .reduce(|x, y| x.intersection(&y).cloned().collect());
            char_to_priority(h1.unwrap().iter().next().unwrap().clone())
        })
        .sum()
}

fn main() {
    const INPUT: &'static str = include_str!("../../input/2022/day3.txt");
    // dbg!(input_generator(INPUT));
    let test_input = "AAAAaABBBBaB
AAzABzBB
AAAABABB
AAZABZBB";
    dbg!(solve_p1(test_input));

    let test_input_pack = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
    dbg!(solve_p2(test_input_pack));
    dbg!(solve_p2(INPUT));
}
