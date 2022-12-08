// use itertools::Itertools;
// use std::collections::HashMap;
// use regex::Regex;
#[derive(Debug)]
pub struct TreeMap(Vec<Vec<Tree>>);
impl TreeMap {
    fn from(input: &str) -> TreeMap {
        TreeMap(
            input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            let height = c.to_digit(10).unwrap();
                            Tree {
                                height: height as isize,
                                visible: false,
                                score: 1,
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<Tree>>>(),
        )
    }
}
#[derive(Debug)]
pub struct Tree {
    height: isize,
    visible: bool,
    score: usize,
}

pub fn solve_p1(input: &str) -> usize {
    let mut map = TreeMap::from(input);
    let DIM = map.0[0].len();
    for i in 0..DIM {
        let mut visibility = -1;
        for j in 0..DIM {
            if map.0[i][j].height > visibility {
                map.0[i][j].visible = true;
                visibility = map.0[i][j].height;
            }
        }
    }
    for i in 0..DIM {
        let mut visibility = -1;
        for j in (0..DIM).rev() {
            if map.0[i][j].height > visibility {
                map.0[i][j].visible = true;
                visibility = map.0[i][j].height;
            }
        }
    }
    for j in 0..DIM {
        let mut visibility = -1;
        for i in 0..DIM {
            if map.0[i][j].height > visibility {
                map.0[i][j].visible = true;
                visibility = map.0[i][j].height;
            }
        }
    }
    for j in 0..DIM {
        let mut visibility = -1;
        for i in (0..DIM).rev() {
            if map.0[i][j].height > visibility {
                map.0[i][j].visible = true;
                visibility = map.0[i][j].height;
            }
        }
    }
    // dbg!(&map);
    map.0
        .iter()
        .flatten()
        .map(|t| if t.visible { 1 } else { 0 })
        .sum()
}

pub fn solve_p2(input: &str) -> usize {
    let mut map = TreeMap::from(input);
    let DIM = map.0[0].len() as isize;
    for i in 1..(DIM - 1) {
        for j in 1..(DIM - 1) {
            let mut score = 0;
            let my_height = map.0[i as usize][j as usize].height;
            for ind in (j + 1)..DIM {
                if map.0[i as usize][ind as usize].height < my_height {
                    score += 1;
                } else {
                    score += 1;
                    break;
                }
            }
            map.0[i as usize][j as usize].score *= score;
            score = 0;
            for ind in (0..=(j - 1)).rev() {
                if map.0[i as usize][ind as usize].height < my_height {
                    score += 1;
                } else {
                    score += 1;
                    break;
                }
            }
            map.0[i as usize][j as usize].score *= score;
            score = 0;
            for ind in (i + 1)..DIM {
                if map.0[ind as usize][j as usize].height < my_height {
                    score += 1;
                } else {
                    score += 1;
                    break;
                }
            }
            map.0[i as usize][j as usize].score *= score;
            score = 0;
            for ind in (0..=(i - 1)).rev() {
                if map.0[ind as usize][j as usize].height < my_height {
                    score += 1;
                } else {
                    score += 1;
                    break;
                }
            }
            map.0[i as usize][j as usize].score *= score;
        }
    }
    let mut map = map.0.iter().flatten().collect::<Vec<_>>();
    map.sort_by_key(|t| -(t.score as isize));
    map[0].score
}

fn main() {
    let test_input = "30373
25512
65332
33549
35390";
    assert_eq!(solve_p1(test_input), 21);
    assert_eq!(solve_p2(test_input), 8);
    const INPUT: &'static str = include_str!("../../input/2022/day8.txt");
    dbg!(solve_p1(INPUT));
    dbg!(solve_p2(INPUT));
}
