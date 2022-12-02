use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn parse_input(filename: &str) -> BufReader<File> {
    let file = File::open(String::from("src/day02/")+filename).unwrap();
    BufReader::new(file)
}

fn part_2(filename: &str) -> u32 {
    iterate(filename, |x, y|
        y.get_outcome_value() + Game::get_game_to_play(x, y).get_value())
}

fn part_1(filename: &str) -> u32 {
    iterate(filename, | x, y|
        y.get_value() + Game::get_win(x, y))
}


fn iterate(filename: &str, calculate_values: fn(&Game, &Game) -> u32) -> u32 {
    parse_input(filename).lines().map(|x| {
        let (player1, player2) : (Game, Game) = x.unwrap().split_whitespace()
            .map(|x| Game::new(x.chars().next().unwrap())).collect_tuple().unwrap();
        // let players : Vec<Game> = x.unwrap().chars().filter(|y| y != &' ')
        //     .map(|x| Game::new(x)).take(2).collect();
        calculate_values(&player1, &player2)
    }).sum()
}

#[derive(PartialEq, Clone, Copy)]
enum Game {
    Rock,
    Paper,
    Scissors
}

impl Game {

    pub fn new(elem: char) -> Self {
        match elem {
            'A' | 'X' => Game::Rock,
            'B' | 'Y' => Game::Paper,
            'C' | 'Z' => Game::Scissors,
            _ => panic!("Invalid input, was {elem}")

        }
    }

    pub fn get_win(opponent: &Game, me:&Game) -> u32 {
        match (opponent, me) {
            (a, b) if a == b => 3,
            (a, b) if b == &Game::get_losing(a) => 0,
            _ => 6
        }
    }

    pub fn get_value(&self) -> u32 {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }

    pub fn get_outcome_value(&self) -> u32 {
        match self {
            Game::Rock => 0,
            Game::Paper => 3,
            Game::Scissors => 6,
        }
    }

    //get the move that the oponent has to play to lose
    fn get_losing(&self) -> Game {
        match self {
            Game::Rock => Game::Scissors,
            Game::Paper => Game::Rock,
            Game::Scissors => Game::Paper
        }
    }

    pub fn get_game_to_play(opponent: &Game, result:&Game) -> Game {
        match (opponent, result) {
            (a, Game::Rock) => a.get_losing(),
            (a, Game::Paper) => *a,
            (a, _) => a.get_losing().get_losing()
        }

    }
}

#[cfg(test)]
mod test{
    use crate::day02::{part_1, part_2};

    #[test]
    fn part_1_example(){
        assert_eq!(15, part_1("test1.txt"));
    }

    #[test]
    fn part_1_puzzle(){
        //print!("{}", part_1("input.txt"));
         assert_eq!(9759, part_1("input.txt"));
    }

    #[test]
    fn part_2_example(){
        assert_eq!(12, part_2("test1.txt"));
    }

    #[test]
    fn part_2_puzzle(){
        //print!("{}", part_2("input.txt"));
        assert_eq!(12429, part_2("input.txt"));
    }
}