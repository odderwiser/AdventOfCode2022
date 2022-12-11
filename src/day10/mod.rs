use itertools::Itertools;

fn part_1(input: &str) -> i32 {
    let mut x_reg = 1;
    let mut cycles = 1;
    let values = vec![20, 60, 100, 140, 180, 220];
    let mut res: i32 = 0;
    input.lines().map(|x| Instr::new(x)).for_each(|x| {
        let cycle = x.get_timing();
        let outcome = x.get_value();
        let new_x = x_reg + outcome;
        if let Some(value) = values
            .iter()
            .filter(|y| &&cycles < y && &(cycles + cycle) >= y)
            .next()
        {
            if &(cycle + cycles) > value {
                res += (*value as i32 * x_reg);
            } else {
                res += *value as i32 * new_x as i32;
            }
        }
        cycles += cycle;
        x_reg = new_x;
    });
    res
}

fn part_2(input: &str) -> String {
    let mut x_reg = 1;
    let mut cycles = 1;
    let mut res = String::new();
    input.lines().map(|x| Instr::new(x)).for_each(|x| {
        let cycle = x.get_timing();
        for i in cycles..(cycles + cycle) {
            if is_pixel_sprite(i - 1, x_reg) {
                res.push('#');
            } else {
                res.push('.')
            }
            if i % 40 == 0 {
                res.push('\n');
            }
        }
        cycles += cycle;
        x_reg += x.get_value();
    });
    res
}

fn is_pixel_sprite(pixel: u8, sprite: i32) -> bool {
    let pix = pixel % 40;
    (sprite - 1..=sprite + 1).contains(&(pix as i32))
}

enum Instr {
    NoOp,
    AddX(i32),
}

impl Instr {
    fn new(input: &str) -> Instr {
        match input {
            x if x.starts_with("noop") => Instr::NoOp,
            x if x.starts_with("addx") => {
                let (a, b) = x.split_whitespace().collect_tuple().unwrap();
                Instr::AddX(b.parse::<i32>().unwrap())
            }
            _ => unreachable!("Invalid instruction!"),
        }
    }

    fn get_timing(&self) -> u8 {
        match self {
            Instr::NoOp => 1,
            Instr::AddX(_) => 2,
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Instr::NoOp => 0,
            Instr::AddX(x) => *x,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day10::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(13140, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(13920, output);
    }

    #[test]
    fn part_2_example1() {
        let result = String::from(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
",
        );
        assert_eq!(result, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        print!("{}", output);
        let result = String::from(
            "####..##..#....#..#.###..#....####...##.
#....#..#.#....#..#.#..#.#....#.......#.
###..#....#....####.###..#....###.....#.
#....#.##.#....#..#.#..#.#....#.......#.
#....#..#.#....#..#.#..#.#....#....#..#.
####..###.####.#..#.###..####.#.....##..
",
        );
        assert_eq!(result, output);
    }
}
