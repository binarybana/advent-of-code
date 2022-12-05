// use itertools::Itertools;
use regex::Regex;

pub fn solve_p1(input: &str) -> usize {
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    input
        .lines()
        .map(|pair| {
            // dbg!(pair);
            let cap = re.captures(pair.trim()).unwrap();
            // dbg!(cap.get(1));
            // dbg!(cap.get(2));
            // dbg!(cap.get(3));
            // dbg!(cap.get(4));
            let (l1, r1) = (
                cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            );
            let (l2, r2) = (
                cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            );
            if (l1 <= l2 && r1 >= r2) || (l2 <= l1 && r2 >= r1) {
                1
            } else {
                0
            }
        })
        .sum()

    //     pair.split(',')
    //         .map(|range| range.split('-').map(|val| val.parse::<usize>().unwrap()))
    //         .map(|x| x[0] + x[1])
    // })
    // .flatten()
    // .sum()
}

pub fn solve_p2(input: &str) -> usize {
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    input
        .lines()
        .map(|pair| {
            let cap = re.captures(pair.trim()).unwrap();
            let (l1, r1) = (
                cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            );
            let (l2, r2) = (
                cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            );
            // max(x1,y1) <= min(x2,y2)
            if l1.max(l2) <= r1.min(r2) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    dbg!(solve_p1(test_input));
    const INPUT: &'static str = include_str!("../../input/2022/day4.txt");
    dbg!(solve_p1(INPUT));
    dbg!(solve_p2(test_input));
    dbg!(solve_p2(INPUT));
}
