use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn part_2(filename: &str) -> u32 {
    let reader = parse_input(filename);
    let mut kcal:u32 = 0;
    let mut elves: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let value = line.unwrap();
        if value.is_empty() {
            elves.push(kcal);
            kcal = 0;
        } else {
            kcal += value.parse::<u32>().unwrap();
        }
    }
    elves.push(kcal);
    elves.sort();
    elves.iter().rev().take(3).sum()
}

fn part_1(filename: &str) -> u32 {
    let reader = parse_input(filename);
    let mut kcal:u32 = 0;
    let mut max: u32 = 0;
    for line in reader.lines() {
        let value = line.unwrap();
        if value.is_empty() {
            if kcal > max {
                max = kcal;
            }
            kcal = 0;
        } else {
            kcal += value.parse::<u32>().unwrap();
        }
    }
    max
}

#[cfg(test)]
mod test{
    use crate::day01::{part_1, part_2};

    #[test]
    fn part_1_example(){
        assert_eq!(24000, part_1("src/day01/test.txt"));
    }

    #[test]
    fn part_1_puzzle(){
        assert_eq!(70613, part_1("src/day01/input.txt"));
    }

    #[test]
    fn part_2_example(){
        assert_eq!(45000, part_2("src/day01/test.txt"));
    }

    #[test]
    fn part_2_puzzle(){
        assert_eq!(205805, part_2("src/day01/input.txt"));
    }
}