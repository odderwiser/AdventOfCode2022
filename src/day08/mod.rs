mod part1;
mod part2;

fn parse(input: &str) -> Vec<Vec<Tree>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| Tree::new(y.to_digit(10).unwrap() as i8))
                .collect::<Vec<Tree>>()
        })
        .collect::<Vec<_>>()
}

fn compute(
    input: &str,
    traversing: impl Fn(&mut dyn Iterator<Item = &mut Vec<Tree>>, bool),
) -> Vec<Vec<Tree>> {
    let mut array = parse(input);
    traversing(&mut array.iter_mut(), false);
    traversing(&mut array.iter_mut(), true);
    array = transpose(array);
    traversing(&mut array.iter_mut(), false);
    traversing(&mut array.iter_mut(), true);
    array
}

//stolen from https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug)]
struct Tree {
    visible: bool,
    height: i8,
    scenic_factor: u32,
}

impl Tree {
    fn new(height: i8) -> Tree {
        Tree {
            visible: false,
            height,
            scenic_factor: 1,
        }
    }

    fn is_visible(&self) -> bool {
        return self.visible;
    }

    fn visible(&mut self) {
        self.visible = true;
    }

    fn scenic_factor(&mut self, see_trees: u32) {
        self.scenic_factor *= see_trees;
    }
}

#[cfg(test)]
mod test {
    use crate::day08::part1::part_1;
    use crate::day08::part2::part_2;

    #[test]
    fn part_1_example() {
        assert_eq!(21, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        print!("{}", output);
        //assert_eq!(1702, output);
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(8, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        print!("{}", output);
        //assert_eq!(3559, output);
    }
}
