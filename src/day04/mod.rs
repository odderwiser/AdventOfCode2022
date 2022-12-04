use itertools::Itertools;

fn part_2(input: &str) -> usize {
    count(input, |a, b| a || b)
}

fn part_1(input: &str) -> usize {
    count(input, |a, b| a && b)
}

fn count(input: &str, logic: impl Fn(bool, bool) -> bool) -> usize {
    input
        .lines()
        .flat_map(|x| {
            x.split([',', '-'])
                .map(|y| y.parse::<usize>().unwrap())
                .tuples()
        })
        .tuples()
        .filter(|((a, b), (c, d))| {
            logic((a..=b).contains(&c), (a..=b).contains(&d))
                || logic((c..=d).contains(&a), (c..=d).contains(&b))
        })
        .count()
}

#[cfg(test)]
mod test {
    use crate::day04::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(2, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(547, output);
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(4, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(843, output);
    }
}
