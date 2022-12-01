use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(filename: &str) -> BufReader<File> {
    let file = File::open(String::from("src/day02/")+filename).unwrap();
    BufReader::new(file)
}

fn part_2(filename: &str) -> () {
    let reader = parse_input(filename);
}

fn part_1(filename: &str) -> () {
    let reader = parse_input(filename);
}

#[cfg(test)]
mod test{
    use crate::day02::{part_1, part_2};

    #[test]
    fn part_1_example(){
        // assert_eq!(24000, part_1("test.txt"));
    }

    #[test]
    fn part_1_puzzle(){
        // assert_eq!(70613, part_1("input.txt"));
    }

    #[test]
    fn part_2_example(){
        // assert_eq!(45000, part_2("test.txt"));
    }

    #[test]
    fn part_2_puzzle(){
        // assert_eq!(205805, part_2("input.txt"));
    }
}