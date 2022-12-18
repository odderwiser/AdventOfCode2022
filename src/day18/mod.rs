use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::convert::identity;
use std::hash::Hash;

fn part_1(input: &str) -> usize {
    let cubes: HashSet<Cube> = HashSet::from_iter(input.lines().map(Cube::new));
    let mut result = 0;
    for cube in cubes.iter() {
        for adjacent_cube in cube.get_directions() {
            if !cubes.contains(&adjacent_cube) {
                result += 1;
            }
        }
    }
    result
}

fn part_1_debug(input: &str) -> Vec<(Cube, usize)> {
    let cubes: HashSet<Cube> = HashSet::from_iter(input.lines().map(Cube::new));
    cubes
        .iter()
        .map(|x| {
            (
                *x,
                x.get_directions()
                    .iter()
                    .filter(|y| !cubes.contains(y))
                    .count(),
            )
        })
        .collect()
}

fn part_2_debug(cubes: &HashSet<Cube>, water: &HashSet<Cube>) -> Vec<(Cube, usize)> {
    cubes
        .iter()
        .map(|x| {
            (
                *x,
                x.get_directions()
                    .iter()
                    .filter(|y| water.contains(y))
                    .count(),
            )
        })
        .collect()
}

fn part_2(input: &str) -> usize {
    let cubes: HashSet<Cube> = HashSet::from_iter(input.lines().map(Cube::new));
    let max_vec = vec![
        cubes.iter().map(|x| x.x).max().unwrap() + 1,
        cubes.iter().map(|x| x.y).max().unwrap() + 1,
        cubes.iter().map(|x| x.z).max().unwrap() + 1,
    ];
    let mut water: HashSet<Cube> = HashSet::new();
    let mut queue = VecDeque::new();
    for i in vec![-1, max_vec[0]] {
        for j in vec![-1, max_vec[1]] {
            for k in vec![-1, max_vec[2]] {
                queue.push_back(Cube::from(i, j, k));
            }
        }
    }
    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();
        water.insert(cube);
        for adjacent_cube in cube.get_directions() {
            if adjacent_cube.to_vec().iter().any(|x| x < &-1)
                || (0..3)
                    .map(|x| adjacent_cube.to_vec()[x] > max_vec[x])
                    .any(identity)
            {
                continue;
            }
            if !cubes.contains(&adjacent_cube)
                && !water.contains(&adjacent_cube)
                && !queue.contains(&adjacent_cube)
            {
                queue.push_back(adjacent_cube);
            }
        }
    }
    let mut result = 0;
    for cube in cubes.iter() {
        for adjacent_cube in cube.get_directions() {
            if water.contains(&adjacent_cube) {
                result += 1;
            }
        }
    }
    result
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn new(input: &str) -> Cube {
        let coords = input
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple::<(_, _, _)>()
            .unwrap();
        Cube {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }

    fn from(x: isize, y: isize, z: isize) -> Cube {
        Cube { x, y, z }
    }

    fn make_cube(&self, coord: isize, change: isize) -> Cube {
        let mut coords = (self.x, self.y, self.z);
        match coord {
            0 => coords.0 += change,
            1 => coords.1 += change,
            2 => coords.2 += change,
            _ => unreachable!(),
        }
        Cube::from(coords.0, coords.1, coords.2)
    }

    fn to_vec(&self) -> Vec<isize> {
        return vec![self.x, self.y, self.z];
    }

    fn get_directions(&self) -> Vec<Cube> {
        let mut res = Vec::new();
        for i in 0..3 {
            res.push(self.make_cube(i, 1));
            res.push(self.make_cube(i, -1));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::day18::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(64, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(4548, output);
    }

    #[test]
    fn part_2_example1() {
        assert_eq!(58, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(6, part_2("1,1,1"));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(2588, output);
    }
}
