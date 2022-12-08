use itertools::Itertools;
use std::collections::HashMap;
// use regex::Regex;

pub fn get_sizes(input: &str) -> HashMap<String, usize> {
    let mut pos: Vec<String> = Vec::new();
    let mut sizes = HashMap::<String, usize>::new();
    for line in input.lines() {
        if line.starts_with("$ cd /") {
            pos.clear();
        } else if line.starts_with("$ cd ..") {
            pos.pop();
        } else if line.starts_with("$ cd ") {
            pos.push(line.trim().split(' ').last().unwrap().into());
        } else if line.starts_with("dir") || line.starts_with("$ ls") {
            continue;
        } else {
            let size = line
                .trim()
                .split(' ')
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let mut pos_clone = pos.clone();
            loop {
                let dir = pos_clone.iter().join(":").clone();
                let entry = sizes.entry(dir).or_insert(0);
                *entry += size;
                let popped = pos_clone.pop();
                if popped.is_none() {
                    break;
                }
            }
        }
    }
    sizes
}
pub fn solve_p1(input: &str) -> usize {
    let sizes = get_sizes(input);
    sizes
        .into_iter()
        .filter_map(|(_, v)| if v <= 100_000 { Some(v) } else { None })
        .sum()
}

const TOTAL_SPACE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;

pub fn solve_p2(input: &str) -> usize {
    let sizes = get_sizes(input);
    let used_space = sizes.get("").unwrap();
    assert!(NEEDED_SPACE > (TOTAL_SPACE - used_space));
    let space_to_clear = NEEDED_SPACE - (TOTAL_SPACE - used_space);
    let mut sizes = sizes.iter().collect::<Vec<_>>();
    sizes.sort_by_key(|(_, &v)| v);
    *sizes.iter().find(|(_, v)| **v > space_to_clear).unwrap().1
}

fn main() {
    let test_input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    assert_eq!(solve_p1(test_input), 95437);
    assert_eq!(solve_p2(test_input), 24933642);
    const INPUT: &'static str = include_str!("../../input/2022/day7.txt");
    dbg!(solve_p1(INPUT));
    dbg!(solve_p2(INPUT));
}
