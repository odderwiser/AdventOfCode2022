use crate::day11::Op::{Add, Multiply, Square};
use itertools::Itertools;
use std::collections::VecDeque;

fn part_1(input: &str) -> usize {
    let (mut monkeys, mut item_queue) = parse(input);
    iterate(&mut monkeys, 20,&mut item_queue, |x| (x / 3) as usize);
    process_result(monkeys)
}

fn part_2(input: &str) -> usize {
    let (mut monkeys, mut item_queue) = parse(input);
    let dividing_factor: usize = monkeys.iter().map(|x| x.test_value).product();
    iterate(&mut monkeys, 10000, &mut item_queue, |x| x % dividing_factor);
    process_result(monkeys)
}

fn parse(input: &str) -> (Vec<Monkey>, Vec<VecDeque<usize>>) {
    let monkeys = input.split("\n\n").map(Monkey::new).collect::<Vec<_>>();
    let mut item_queue: Vec<VecDeque<usize>> = Vec::new();
    (0..monkeys.len()).for_each(|_| item_queue.push(VecDeque::new()));
    (monkeys, item_queue)
}

fn iterate(
    monkeys: &mut [Monkey],
    loops: usize,
    item_queue: &mut [VecDeque<usize>],
    manage_worry: impl Fn(usize) -> usize,
) {
    (0..loops).for_each(|_| monkeys.iter_mut().for_each(|monkey| {
        while !item_queue[monkey.id].is_empty() {
            monkey.add(item_queue[monkey.id].pop_front().unwrap());
        }
        let items = monkey.make_round(&manage_worry);
        for (item, rec) in items {
            item_queue[rec].push_back(item);
        }
    }));
}

fn process_result(monkeys: Vec<Monkey>) -> usize {
    let businesses = monkeys
        .iter()
        .map(|x| x.inspected_items)
        .sorted()
        .tuple_windows::<(_, _)>()
        .last()
        .unwrap();
    businesses.0 * businesses.1
}

struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    inspect: Op,
    test_value: usize,
    monkey_true: u8,
    monkey_false: u8,
    inspected_items: usize,
}

enum Op {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Op {
    fn inspect(&self, value: usize) -> usize {
        match self {
            Add(factor) => value + factor,
            Multiply(factor) => value * factor,
            Square => value * value,
        }
    }
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        let data = input.lines().collect::<Vec<_>>();
        let id = data[0]
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let items = data[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<VecDeque<_>>();
        let (op, val) = data[2]
            .split("old ")
            .last()
            .unwrap()
            .split_whitespace()
            .collect_tuple::<(_, _)>()
            .unwrap();
        let inspect = match (op, val) {
            ("*", "old") => Square,
            ("*", _) => Multiply(val.parse::<usize>().unwrap()),
            ("+", _) => Add(val.parse::<usize>().unwrap()),
            _ => unreachable!("invalid operation: {}", op),
        };
        Monkey {
            id,
            items,
            inspect,
            test_value: Monkey::get_last(data[3]) as usize,
            monkey_true: Monkey::get_last(data[4]),
            monkey_false: Monkey::get_last(data[5]),
            inspected_items: 0,
        }
    }

    fn get_last(line: &str) -> u8 {
        line.split_whitespace()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap()
    }

    fn process_item(&mut self, manage_worry: &impl Fn(usize) -> usize) -> (usize, usize) {
        let mut item = self.items.pop_front().unwrap();
        item = self.inspect.inspect(item);
        item = manage_worry(item);
        let monkey = if (item % self.test_value) == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        };
        self.inspected_items += 1;
        (item, monkey as usize)
    }

    fn make_round(&mut self, manage_worry: impl Fn(usize) -> usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        while !self.items.is_empty() {
            result.push(self.process_item(&manage_worry));
        }
        result
    }

    fn add(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

#[cfg(test)]
mod test {
    use crate::day11::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(10605, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        print!("{}", output);
        //assert_eq!(13920, output);
    }

    #[test]
    fn part_2_example1() {
        assert_eq!(2713310158, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        print!("{}", output);
        //assert_eq!(result, output);
    }
}
