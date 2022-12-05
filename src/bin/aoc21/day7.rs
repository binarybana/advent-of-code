const INPUT: &'static str = include_str!("../input/2021/day7.txt");

pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|pos| pos.parse::<usize>().unwrap())
        .collect()
}

pub fn solve_part1(input: Vec<usize>) -> usize {
    let mut nums = ndarray::Array::from_vec(input);
    use ndarray_stats::Quantile1dExt;
    use noisy_float::types::n64;
    let median = nums
        .quantile_mut(n64(0.5), &ndarray_stats::interpolate::Lower)
        .expect("Couldn't calcualte median");
    nums.iter()
        .map(|x| (*x as isize - median as isize).abs())
        .sum::<isize>() as usize
}

pub fn solve_part2(input: Vec<usize>) -> usize {
    let nums = ndarray::Array::from_vec(input.iter().map(|x| *x as f64).collect());
    let mean = nums.mean().expect("Couldn't calcualte mean");
    dbg!(mean);
    nums.iter()
        .map(|x| {
            let x = ((*x).round() - mean.floor()).abs();
            x * (x + 1.0) / 2.0
        })
        .sum::<f64>()
        .round() as usize
}

#[test]
fn test() {
    // dbg!(input_generator(INPUT));
    // dbg!(solve_part1(input_generator(INPUT)));
    // dbg!(solve_part2(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]));
    dbg!(solve_part2(input_generator(INPUT)));
    // 96708205
    // tried 96708215, but wrong
    // assert_eq!(calc_children(9, 80), 2);
    // assert_eq!(solve_part1(&vec![3, 4, 3, 1, 2], 18), 26);
    // assert_eq!(solve_part1(&vec![3, 4, 3, 1, 2], 80), 5934);
    // dbg!(solve_part1(&input_generator(INPUT), 80));
    // assert_eq!(solve_part1(&vec![3, 4, 3, 1, 2], 256), 26984457539);
    // dbg!(solve_part1(&input_generator(INPUT), 256));
}
