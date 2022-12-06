use std::collections::HashSet;

fn part_2(input: &str) -> usize {
    seq_index(input, 14)
}

fn part_1(input: &str) -> usize {
    seq_index(input, 4)
}

fn seq_index(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .map(|x| {
            let hashed: HashSet<&char> = HashSet::from_iter(x.iter());
            hashed.len()
        })
        .enumerate()
        .find(|(_, length): &(usize, usize)| length == &window_size)
        .unwrap()
        .0
        + window_size
}

#[cfg(test)]
mod test {
    use crate::day06::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(11, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(1702, output);
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(26, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(3559, output);
    }
}
