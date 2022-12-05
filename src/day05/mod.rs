mod part1;
mod part2;

use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

fn make_result(stacks: Vec<VecDeque<char>>) -> String {
    let mut result: Vec<char> = Vec::new();
    stacks
        .iter()
        .for_each(|stack| result.push(*stack.back().unwrap()));
    result.iter().collect()
}

fn parse_moves(input: &str) -> Box<dyn Iterator<Item = (usize, usize, usize)> + '_> {
    let regex = Regex::new(r" from | to ").unwrap();
    Box::new(input.lines().map(|x| &x[5..]).map(move |x| {
        regex
            .split(x)
            .map(|y| y.parse::<usize>().unwrap())
            .tuples()
            .next()
            .unwrap()
    }))
}

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .for_each(|line| {
            for i in (1..line.len()).step_by(4) {
                let index = (i - 1) / 4;
                let character = line.get(i).unwrap();
                if character != &' ' {
                    while index >= stacks.len() {
                        stacks.push(VecDeque::new())
                    }
                    let stack1: &mut VecDeque<char> = stacks.get_mut(index).unwrap();
                    stack1.push_front(*character);
                }
            }
        });
    stacks
}

fn parse_input(
    input: &str,
) -> (
    Vec<VecDeque<char>>,
    Box<dyn Iterator<Item = (usize, usize, usize)> + '_>,
) {
    let (stack, moves) = input.split("\n\n").tuples().next().unwrap();
    let stacks: Vec<VecDeque<char>> = parse_stacks(stack);
    (stacks, parse_moves(moves))
}
