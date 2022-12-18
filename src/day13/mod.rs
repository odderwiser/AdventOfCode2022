use itertools::Itertools;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::ops::Add;

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x| x.lines().map(parse_package).collect_tuple::<(_, _)>())
        .map(|x| x.unwrap())
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn compare_packets(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        (Packet::List(a), Packet::List(b)) => {
            for i in 0..usize::min(a.len(), b.len()) {
                match compare_packets(&a[i], &b[i]) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            a.len().cmp(&b.len())
        }
        (Packet::Number(a), Packet::List(_)) => {
            compare_packets(&Packet::List(vec![Packet::Number(*a)]), right)
        }
        (Packet::List(_), Packet::Number(b)) => {
            compare_packets(left, &Packet::List(vec![Packet::Number(*b)]))
        }
    }
}

#[derive(Debug, Eq, Ord)]
enum Packet {
    List(Vec<Packet>),
    Number(u8),
}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        compare_packets(&self, &other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(compare_packets(&self, &other))
    }
}

fn part_2(input: &str) -> usize {
    let dividers = String::from("[[2]]\n[[6]]");
    let divider_packets = dividers.lines().map(parse_package).collect::<Vec<Packet>>();
    let packets = dividers
        .add("\n\n")
        .add(input)
        .split("\n\n")
        .flat_map(|x| x.lines().map(parse_package).collect::<Vec<Packet>>())
        .sorted()
        .enumerate()
        .filter(|x| divider_packets.contains(&x.1))
        .map(|x| x.0 + 1)
        .product();

    packets
}

fn parse_package(packet: &str) -> Packet {
    parse_packet(&mut packet.chars().peekable())
}

fn parse_packet(packet: &mut Peekable<impl Iterator<Item = char>>) -> Packet {
    let mut list: Vec<Packet> = Vec::new();
    assert_eq!(packet.peek().unwrap(), &'[');
    packet.next().unwrap(); //remove the extra bracket
    while packet.peek().is_some() && packet.peek().unwrap() != &']' {
        //parse as long as it is not closed bracket
        //println!("Next char to process: {}", packet.peek().unwrap());
        match packet.peek().unwrap() {
            ',' => {
                packet.next().unwrap();
            }
            '[' => list.push(parse_packet(packet)),
            _ => {
                let mut chars = String::new();
                while packet.peek().unwrap().is_numeric() {
                    chars.push(packet.next().unwrap())
                }
                list.push(Packet::Number(chars.parse().unwrap()));
            }
        }
    }
    assert_eq!(packet.peek().unwrap(), &']');
    packet.next().unwrap();
    Packet::List(list)
}

fn sorting_wrapper() {}

#[cfg(test)]
mod test {
    use crate::day13::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(13, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(6484, output);
    }

    #[test]
    fn part_2_example1() {
        assert_eq!(140, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(19305, output);
    }
}
