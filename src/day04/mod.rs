use itertools::Itertools;

fn part_2(input: &str) -> usize {
    count(input, |a, b| a || b)
}

fn part_1(input: &str) -> usize {
    count(input, |a, b| a && b)
}

fn count(input: &str, logic: impl Fn(bool, bool) -> bool) -> usize {
    input
        .split([',', '-', '\n'])
        .map(|y| y.parse::<usize>().unwrap())
        .tuples()
        .filter(|(a, b, c, d)| {
            logic((a..=b).contains(&c), (a..=b).contains(&d))
                || logic((c..=d).contains(&a), (c..=d).contains(&b))
        })
        .count()
}

fn oneliner() {
     [|a, b| a &&b, |a, b| a || b].iter().map(|x|
         include_str!("input.txt").split([',', '-', '\n'])
             .map(|y| y.parse::<usize>().unwrap())
             .tuples()
             .filter(|(a, b, c, d)|
                 x((a..=b).contains(&c), (a..=b).contains(&d))
                     || x((c..=d).contains(&a), (c..=d).contains(&b))
             )
             .count()
     ).for_each(|x|println!("{x}"))
}

#[cfg(test)]
mod test {
    use crate::day04::{oneliner, part_1, part_2};

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

    #[test]
    fn test_oneliner() {
        oneliner()
    }
}
