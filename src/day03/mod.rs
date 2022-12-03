use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Flatten;

fn parse_input(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(String::from("src/day03/") + filename).unwrap();
    BufReader::new(file).lines().flatten()
}

fn part_2(filename: &str) -> usize {
    parse_input(filename)
        .chunks(3)
        .into_iter()
        .map(|x| x.collect_tuple().unwrap())
        .map(|x: (String, String, String)| {
            get_value(
                x.0.chars()
                    .find(|y| x.1.contains(*y) && x.2.contains(*y))
                    .unwrap(),
            ) as usize
        })
        .sum()
}

fn part_1(filename: &str) -> usize {
    parse_input(filename)
        .map(|line| {
            let sets = line.split_at(line.len() / 2);
            get_value(
                sets.0
                    .chars()
                    .find(|x| sets.1.contains(*x))
                    .unwrap(),
            ) as usize
        })
        .sum()
}

fn get_value(c: char) -> u8 {
    match c {
        a if a.is_lowercase() => a as u8 - b'a' + 1,
        a => a as u8 - b'A'+ 27,
    }
}

#[cfg(test)]
mod test {
    use crate::day03::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(157, part_1("test1.txt"));
    }

    #[test]
    fn part_1_puzzle() {
        //print!("{}", part_1("input.txt"));
        assert_eq!(8123, part_1("input.txt"));
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(70, part_2("test1.txt"));
    }

    #[test]
    fn part_2_puzzle() {
        //print!("{}", part_2("input.txt"));
        assert_eq!(2620, part_2("input.txt"));
    }
}
