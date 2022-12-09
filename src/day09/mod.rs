use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
use std::{iter, num};

fn parse(input: &str) -> Box<dyn Iterator<Item = Move> +'_> {
    Box::new(input.lines().map(|x| {
        x.split_whitespace()
            .tuples::<(_, _)>().next().unwrap()} )
        .map(|(a, b)| Move::new(a.parse::<char>().unwrap(), b.parse::<u16>().unwrap())
    ))
}

fn part_1(input: &str) -> usize {
    let moves : Box<dyn Iterator<Item=Move> >= parse(input);
    let generated = generate(moves);
    HashSet::<Pos>::from_iter(generated).len()
}

fn part_2(input: &str) -> usize {
    let moves : Box<dyn Iterator<Item=Move> > = parse(input);
    let mut generated = generate(moves);
    for _ in 2..=9 {
        generated = iterate(generated);
    }
    HashSet::<Pos>::from_iter(generated).len()
}

fn generate<'a>(moves : Box<dyn Iterator<Item = Move>+ 'a>) -> Box<dyn Iterator<Item = Pos> + 'a> {
    let mut start = Pos::new(0,0);
    let mut follower : Pos = Pos::new(0,0);
    Box::new(iter::once(Pos::new(0,0)).chain(moves
        .map(move |x: Move| x.make_move(&mut start, &mut follower))
        .flatten()))
}

fn iterate<'a>(generated : Box<dyn Iterator<Item = Pos>+'a>) -> Box<dyn Iterator<Item = Pos> + 'a> {
    let mut follower = Pos::new(0,0);
    Box::new(generated
        .map(move |x| {
            follower.make_step(&x);
            follower.clone()
        }).dedup())
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Move {
    direction: Dir,
    steps: u16,
}

#[derive(Debug)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

impl Dir {
    fn new(dir: char) -> Dir {
        match dir {
            'R' => Dir::Right,
            'L' => Dir::Left,
            'U' => Dir::Up,
            'D' => Dir::Down,
            _ => unreachable!(),
        }
    }
}

impl Move {
    fn new(dir: char, val: u16) -> Move {
        Move {
            direction: Dir::new(dir),
            steps: val,
        }
    }

    fn get_step_dir(&self) -> Pos {
        match self.direction {
            Dir::Right => Pos::new(1, 0),
            Dir::Left => Pos::new(-1, 0),
            Dir::Down => Pos::new(0, -1),
            Dir::Up => Pos::new(-0, 1),
        }
    }

    fn make_move(&self, head: &mut Pos, tail: &mut Pos) -> Vec<Pos> {
        //println!("Move is: {:?}", self);
        let mut res = Vec::new();
        for i in 0..self.steps {
            head.update(self.get_step_dir());
            if !tail.is_adjacent(head) {
                tail.move_towards(head);
                res.push(tail.clone());
            }
            //println!("Tail is: {:?}, head is: {:?}", tail, head);
        }
        res
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn update(&mut self, other: Pos) {
        self.x += other.x;
        self.y += other.y;
    }

    fn is_adjacent(&self, other: &Pos) -> bool {
        i32::abs(self.x - other.x) <= 1 && i32::abs(self.y - other.y) <= 1
    }

    fn move_towards(&mut self, other: &Pos) {
        match (other.x - self.x, other.y - self.y) {
            (a, b) if self.is_adjacent(other) => (),
            (a, b) if a == 0 => self.y += (b / 2),
            (a, b) if b == 0 => self.x += (a / 2),
            (a, b) if i32::abs(a) == 1 => {
                self.x += a;
                self.y += (b / 2);
            }
            (a, b) if i32::abs(b) == 1 => {
                self.x += (a / 2);
                self.y += b;
            }
            (a, b) if i32::abs(a) == i32::abs(b) => {
                self.x += a/2;
                self.y += b/2;
            }
            _ => unreachable!("Not supposed to happen! Following: {:?}, current: {:?}", other, self),
        }
    }

    fn make_step(&mut self, next: &Pos) {
        if !self.is_adjacent(next) {
            self.move_towards(next);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day09::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(13, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(6314, output);
    }

    #[test]
    fn part_2_example1() {
        assert_eq!(1, part_2(include_str!("test1.txt")));
    }
    #[test]
    fn part_2_example2() {
        assert_eq!(36, part_2(include_str!("test2.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(2504, output);
    }
}