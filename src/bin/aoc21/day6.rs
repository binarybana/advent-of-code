const INPUT: &'static str = include_str!("../input/2021/day6.txt");

type School = Vec<usize>;

pub fn input_generator(input: &str) -> School {
    input
        .trim()
        .split(',')
        .map(|age| age.parse::<usize>().unwrap())
        .collect()
}

/// For a single fish that is <age> away from reproducing,
/// then calculate the number of it's children <days> days from now
use cached::proc_macro::cached;
#[cached]
fn calc_children(age: usize, days: usize) -> usize {
    // use std::cell::RefCell;
    // use std::collections::HashMap;
    // thread_local! {
    //     static memomap: RefCell<HashMap<(usize,usize), usize>> = Default::default();
    // }
    // static mut memomap: OnceCell<HashMap<(usize, usize), usize>> = HashMap::new();
    if days <= age {
        1 // count myself, but I'm not going to have any more children
    } else {
        // I'm going to have at least one child, so let's fast forward to that time
        // let new_days = days - age - 1;
        // let me = if let Some(num) = memomap.with(|memomapi| {
        //     memomapi.borrow_mut().get((6, new_days))) {

        //     }
        //     let me = memomapi
        //         .entry((6, new_days))
        //         .or_insert_with(|| calc_children(6, new_days));
        //     let mut memomapii = memomapi.borrow_mut();
        //     let new_baby = memomapii
        //         .entry((8, new_days))
        //         .or_insert_with(|| calc_children(8, new_days));
        //     *new_baby + *me
        // })
        // let new_days = dbg!(days - age - 1);
        // dbg!(dbg!(calc_children(8, new_days)) + dbg!(calc_children(6, new_days)))
        let new_days = days - age - 1;
        calc_children(8, new_days) + calc_children(6, new_days)
    }
}

pub fn solve_part1(input: &School, days: usize) -> usize {
    input.iter().map(|i| calc_children(*i, days)).sum()
}

#[test]
fn test() {
    // dbg!(input_generator(INPUT));
    // assert_eq!(calc_children(8, 7), 1);
    // assert_eq!(calc_children(8, 8), 1);
    // assert_eq!(calc_children(8, 9), 2);

    //  0: 0
    //  1: 6, 8
    //  2: 5, 7
    //  3: 4, 6
    //  4: 3, 5
    //  5: 2, 4
    //  6: 1, 3
    //  7: 0, 2
    //  8: 6, 1, 8
    //  9: 5, 0, 7
    // 10: 4, 6, 6, 8
    assert_eq!(calc_children(0, 0), 1);
    assert_eq!(calc_children(0, 1), 2);
    assert_eq!(calc_children(0, 7), 2);
    assert_eq!(calc_children(0, 8), 3);
    assert_eq!(calc_children(0, 9), 3);
    assert_eq!(calc_children(0, 10), 4);

    // now plus 1 age offsets
    //  0: 1
    //  1: 0
    //  2: 6, 8
    //  3: 5, 7
    //  4: 4, 6
    //  5: 3, 5
    //  6: 2, 4
    //  7: 1, 3
    //  8: 0, 2
    //  9: 6, 1, 8
    // 10: 5, 0, 7
    // 11: 4, 6, 6, 8
    assert_eq!(calc_children(1, 1), 1);
    assert_eq!(calc_children(1, 2), 2);
    assert_eq!(calc_children(1, 8), 2);
    assert_eq!(calc_children(1, 9), 3);
    assert_eq!(calc_children(1, 10), 3);
    assert_eq!(calc_children(1, 11), 4);
    // assert_eq!(calc_children(8, 17), 4);
    // assert_eq!(calc_children(9, 80), 2);
    assert_eq!(solve_part1(&vec![3, 4, 3, 1, 2], 18), 26);
    assert_eq!(solve_part1(&vec![3, 4, 3, 1, 2], 80), 5934);
    dbg!(solve_part1(&input_generator(INPUT), 80));
    assert_eq!(solve_part1(&vec![3, 4, 3, 1, 2], 256), 26984457539);
    dbg!(solve_part1(&input_generator(INPUT), 256));
}
