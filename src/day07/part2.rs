use crate::day07::{parse, process_folder, recurse};
use itertools::Itertools;

pub fn part_2(input: &str) -> usize {
    let mut result = Vec::new();
    let total_size = recurse(&mut parse(input), &mut result);
    *result
        .iter()
        .sorted()
        .find(|x| 70000000 - total_size + *x >= 30000000)
        .unwrap()
}

