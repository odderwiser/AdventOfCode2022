use std::hash::Hash;
use itertools::Itertools;
use std::iter;

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
    generated.sorted().dedup().count()
}

fn part_2(input: &str) -> usize {
    let moves : Box<dyn Iterator<Item=Move> > = parse(input);
    let mut generated = generate(moves);
    for _ in 2..=9 {
        generated = iterate(generated);
    }
    generated.sorted().dedup().count()+1 // +1 because flatmap removes a starting position
}

fn generate<'a>(moves : Box<dyn Iterator<Item = Move>+ 'a>) -> Box<dyn Iterator<Item = Pos> + 'a> {
    let mut start = Pos::new(0,0);
    let mut follower : Pos = Pos::new(0,0);
    Box::new(iter::once(Pos::new(0,0)).chain(moves
        .flat_map(move |x: Move| x.make_move(&mut start, &mut follower))))
}

fn iterate<'a>(generated : Box<dyn Iterator<Item = Pos>+'a>) -> Box<dyn Iterator<Item = Pos> + 'a> {
    let mut follower = Pos::new(0,0);
    Box::new(generated
        .filter_map(move |x| {
            if follower.is_adjacent(&x) {
                None
            } else {
                follower.move_towards(&x);
                Some(follower.clone())
            }
        }))
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
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
        let mut res = Vec::new();
        for _ in 0..self.steps {
            head.update(self.get_step_dir());
            if !tail.is_adjacent(head) {
                tail.move_towards(head);
                res.push(tail.clone());
            }
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
            (_, _) if self.is_adjacent(other) => (),
            (a, b) => {
                self.x +=a.signum();
                self.y+=b.signum();
            }
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
