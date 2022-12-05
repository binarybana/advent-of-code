// use itertools::Itertools;
use regex::Regex;

type Stack = Vec<Vec<char>>;

pub fn execute_move(
    stacks: &mut Stack,
    from: usize,
    to: usize,
    num_blocks: usize,
    retain_order: bool,
) {
    if retain_order {
        // dbg!(&stacks, num_blocks, from, to);
        let split_pt = stacks[from - 1].len() - num_blocks;
        let blocks = stacks[from - 1].split_off(split_pt);
        stacks[to - 1].extend(blocks);
        // dbg!(&stacks);
    } else {
        for _ in 0..num_blocks {
            let grabbed = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(grabbed)
        }
    }
}

pub fn solve_p1(input: &str, retain_order: bool) -> String {
    let input: Vec<_> = input.trim_end().split('\n').collect();
    let line_len = input[0].len();
    let num_stacks = (line_len - 2) / 4 + 1;

    let mut stacks = vec![Vec::with_capacity(50); num_stacks];

    for line in input.iter() {
        if !(line.starts_with("  ") || line.starts_with('[')) {
            // dbg!("skipping ", line);
            continue;
        }
        for stack_ind in 0..num_stacks {
            let box_label = line.as_bytes()[1 + stack_ind * 4] as char;
            if box_label != ' ' {
                // dbg!("adding ", box_label, &stacks, "from ", line);
                stacks[stack_ind].push(box_label)
            }
        }
    }
    for ind in 0..num_stacks {
        stacks[ind].reverse();
    }
    // dbg!(&stacks);
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    for instruction in input.iter().skip_while(|line| !line.starts_with("move")) {
        // dbg!(instruction);
        let caps = re.captures(instruction).unwrap();
        let num_blocks = caps
            .get(1)
            .and_then(|x| x.as_str().parse::<usize>().ok())
            .unwrap();
        let from = caps
            .get(2)
            .and_then(|x| x.as_str().parse::<usize>().ok())
            .unwrap();
        let to = caps
            .get(3)
            .and_then(|x| x.as_str().parse::<usize>().ok())
            .unwrap();

        execute_move(&mut stacks, from, to, num_blocks, retain_order)
    }
    dbg!(&stacks);
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

pub fn solve_p2(input: &str) -> usize {
    0
}

fn main() {
    let test_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    const INPUT: &'static str = include_str!("../../input/2022/day5.txt");
    // dbg!(solve_p1(test_input, false));
    // dbg!(solve_p1(INPUT, false));
    dbg!(solve_p1(test_input, true));
    dbg!(solve_p1(INPUT, true));
}
