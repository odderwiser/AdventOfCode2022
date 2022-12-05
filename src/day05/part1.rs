use crate::day05::{make_result, parse_input};

fn part_1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    moves.for_each(|(a, b, c)| {
        for _ in 0..a {
            let value = stacks.get_mut(b - 1).unwrap().pop_back().unwrap();
            stacks.get_mut(c - 1).unwrap().push_back(value);
        }
    });
    make_result(stacks)
}

#[cfg(test)]
mod test {
    use crate::day05::part1::part_1;

    #[test]
    fn part_1_example() {
        assert_eq!("CMZ", part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!("ZWHVFWQWW", output);
    }
}
