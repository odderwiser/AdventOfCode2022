use crate::day07::{parse, recurse};

pub fn part_1(input: &str) -> usize {
    let mut result = Vec::new();
    recurse(&mut parse(input), &mut result);
    result
        .iter()
        .filter(|x| *x < &(100000 as usize))
        .sum()
}
