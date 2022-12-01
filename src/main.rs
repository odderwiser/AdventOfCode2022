use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input_01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut kcal:u64 = 0;
    let mut elves: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let value = line.unwrap();
        if value.is_empty() {
           elves.push(kcal);
            kcal = 0;
        } else {
            kcal += value.parse::<u64>().unwrap();
        }
    }
    elves.push(kcal);
    elves.sort();
    elves.reverse();
    let max : u64 = elves.iter().take(3).sum();
    println!("{max}");
}

fn part_1(reader: BufReader<File>) -> u64 {
    let mut kcal:u64 = 0;
    let mut max: u64 = 0;
    for line in reader.lines() {
        let value = line.unwrap();
        if value.is_empty() {
            if kcal > max {
                max = kcal;
            }
            kcal = 0;
        } else {
            kcal += value.parse::<u64>().unwrap();
        }
    }
    return max;
}
