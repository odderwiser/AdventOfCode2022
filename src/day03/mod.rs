use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(filename: &str) -> BufReader<File> {
    let file = File::open(String::from("src/day03/") + filename).unwrap();
    BufReader::new(file)
}

fn part_2(filename: &str) -> u32 {
    parse_input(filename)
        .lines()
        .map(|x| x.unwrap())
        .chunks(3)
        .into_iter()
        .map(|x| x.collect_tuple().unwrap())
        .map(|x: (String, String, String)| {
            get_value(
                x.0.chars()
                    .filter(|y| x.1.contains(*y) && x.2.contains(*y))
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}

fn part_1(filename: &str) -> u32 {
    parse_input(filename)
        .lines()
        .map(|line| {
            let value = line.unwrap();
            let sets = value.split_at(value.len() / 2);
            get_value(
                sets.0
                    .chars()
                    .filter(|x| sets.1.contains(*x))
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}

fn get_value(c: char) -> u32 {
    match c {
        a if a.is_lowercase() => a as u32 - 96,
        a => a as u32 - 65 + 27,
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
