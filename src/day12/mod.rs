use std::cmp::Reverse;
use std::collections::HashSet;
use std::hash::Hash;
use priority_queue::PriorityQueue;


fn part_1(input: &str) -> usize {
    let mut start = (0,0);
    let lined  = input.lines().enumerate().map(|(y, line)| {
        if let Some((x, z)) =  line.chars().enumerate().find(|(x, z)| *z == 'S') {
            start = (x, y);
        }
        line.chars().enumerate().map(|x| Pos::new(x.0, y, x.1)).collect::<Vec<_>>()
    }).collect::<Vec<Vec<Pos>>>();
    let mut queue : PriorityQueue<&Pos, Reverse<usize>> = PriorityQueue::new();
    queue.push(&lined[start.1][start.0], Reverse(0));
    dijkstra(&mut queue, &lined).unwrap()
}

fn dijkstra<'a>(queue: &mut PriorityQueue<&'a Pos, Reverse<usize>>, lined: &'a Vec<Vec<Pos>>) -> Option<usize> {
    let mut visited :HashSet<&(usize, usize)>= HashSet::new();
    queue.iter().for_each(|x| {
        visited.insert(&x.0.coords);
    });
    while queue.peek().is_some() {
        let place = queue.pop().unwrap();
        if place.0.height == 'E' {
            return Some(place.1.0);
        }
        visited.insert(&place.0.coords);
        let dirs_list = place.0.get_dir(lined.len(), lined[0].len());
        for (x, y) in dirs_list {
            let pos = &lined[y][x];
            if !visited.contains(&pos.coords) && place.0.is_reachable(&pos) {
                queue.push(pos, Reverse(place.1.0+1));
                visited.insert(&pos.coords);
            }
        }
    }
    None
}


fn part_2(input: &str) -> usize {
    let mut queue : PriorityQueue<&Pos, Reverse<usize>> = PriorityQueue::new();
    let lined  = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|x| Pos::new(x.0, y, x.1)).collect::<Vec<_>>()
    }).collect::<Vec<Vec<Pos>>>();
    for line in lined.as_slice() {
        for pos in line {
            if pos.height == 'a' {
                queue.push(&pos, Reverse(0));
            }
        }
    }
    return dijkstra(&mut queue, &lined).unwrap();
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Pos {
    height: char,
    coords: (usize, usize)
}

impl Pos {
    fn new(x: usize, y: usize, pos: char) -> Pos{
        Pos{
            height: pos,
            coords: (x, y)
        }
    }

    fn is_reachable(&self, other: &Pos) -> bool {
        match (self.height, other.height) {
            ('S', a) => a as u8 <= 'b' as u8,
            (z, 'E') =>  z == 'z' || z == 'y',
            (from, to) => from as u8 >= to as u8 || to as u8 - from as u8 <= 1
        }
    }

    fn get_dir(&self,  height: usize, width: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if self.coords.0 > 0 {
            result.push((self.coords.0-1, self.coords.1))
        }
        if self.coords.0 < width-1 {
            result.push((self.coords.0+1, self.coords.1))
        }
        if self.coords.1 > 0 {
            result.push((self.coords.0, self.coords.1-1))
        }
        if self.coords.1 < height-1 {
            result.push((self.coords.0, self.coords.1+1))
        }
        result
    }
}


#[cfg(test)]
mod test {
    use crate::day12::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(31, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(420, output);
    }

    #[test]
    fn part_2_example1() {
        assert_eq!(29   , part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(414, output);
    }
}
