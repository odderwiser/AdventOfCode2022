use crate::day05::{make_result, parse_input};
use std::collections::VecDeque;

fn part_2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    moves.for_each(|(a, b, c)| {
        let mut temp: VecDeque<char> = VecDeque::new();
        for _ in 0..a {
            let value = stacks.get_mut(b - 1).unwrap().pop_back().unwrap();
            temp.push_front(value);
        }
        for elem in temp {
            stacks.get_mut(c - 1).unwrap().push_back(elem);
        }
    });
    make_result(stacks)
}

#[cfg(test)]
mod test {
    use crate::day05::part2::part_2;

    #[test]
    fn part_2_example2() {
        assert_eq!("MCD", part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!("HZFZCCWWV", output);
    }
}
