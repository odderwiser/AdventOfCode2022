use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(filename: &str) -> impl Iterator<Item = &str> {
    filename.lines()
}

fn part_2(input: &str) ->  usize {
    parse_input(input);
    0
}

fn part_1(input: &str) -> usize {
    parse_input(input);
    0
}


#[cfg(test)]
mod test {
    use crate::day04::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(0, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        //print!("{}", part_1("input.txt"));
        assert_eq!(0, part_1(include_str!("input.txt")));
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(0, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        //print!("{}", part_2("input.txt"));
        assert_eq!(0, part_2(include_str!("input.txt")));
    }
}
