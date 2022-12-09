use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
use std::{iter, num};

fn parse(input: &str) -> Box<dyn Iterator<Item = Move> + '_> {
    Box::new(input.lines().map(|x| {
        x.split_whitespace()
            .tuples::<(_, _)>().next().unwrap()} )
        .map(|(a, b)| Move::new(a.parse::<char>().unwrap(), b.parse::<u16>().unwrap())
    ))
}

fn part_1(input: &str) -> usize {
    let moves : Box<dyn Iterator<Item=Move> >= parse(input);
    let (mut start_head, mut start_tail) = (Pos::new(0, 0), Pos::new(0, 0));
    let mut positions = moves
        .map(|x: Move| x.make_move(&mut start_head, &mut start_tail))
        .flatten().collect::<HashSet<Pos>>();
    positions.insert(Pos::new(0,0));
    positions.len()
}

fn part_2(input: &str) -> usize {
    let moves : Box<dyn Iterator<Item=Move> > = parse(input);
    let mut start = Pos::new(0,0);
    let mut follower : Pos = Pos::new(0,0);
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut generated : Box<dyn Iterator<Item = Pos>> = Box::new(iter::once(Pos::new(0,0)).chain(moves
        .map(|x: Move| x.make_move(&mut start, &mut follower))
        .flatten().filter(|x| {
        if !visited.contains(x) {
            visited.insert(x.clone());
            return true
        }
        false
    })));
    for i in 1..8 {
        generated = iterate(generated);
    }
    generated.count()
}

fn iterate(generated : Box<dyn Iterator<Item = Pos>>) -> Box<dyn Iterator<Item = Pos>> {
    let follower = Pos::new(0,0);
    let mut visited = HashSet::new();
    Box::new(generated
        .map(move |x| {
            let mut new_follower = follower.clone();
            new_follower.make_step(&x);
            new_follower
        }).filter(move |x| {
        if !visited.contains(x) {
            visited.insert(x.clone());
            return true
        }
        false
    }))
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
        print!("{}", output);
        //assert_eq!(1702, output);
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(0, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        print!("{}", output);
        //assert_eq!(3559, output);
    }
}
