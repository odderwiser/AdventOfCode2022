use itertools::Itertools;

fn part_1(input: &str) -> usize {
    let mut result = 0;
    recurse(
        &mut input
            .lines()
            .filter(|x| x.starts_with('$') || x.chars().next().unwrap().is_numeric()),
        &mut result,
    );
    result
}

//this function should iterate over one folder
fn recurse(input: &mut dyn Iterator<Item = &str>, total_sum: &mut usize) -> usize {
    let mut local_sum = 0;
    let mut value = input.next();
    while value.is_some() {
        println!("Matching in main loop: {}", value.unwrap());
        match value.unwrap() {
            "$ cd .. " => {
                if total_sum < &mut (100000 as usize) {
                    *total_sum += local_sum;
                }
                return local_sum;
            }
            "$ ls" => {
                value = input.next();
                while value.is_some() && !value.unwrap().starts_with('$') {
                    println!("Matching in ls loop: {}", value.unwrap());
                    local_sum += value
                        .unwrap()
                        .split(' ')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    value = input.next();
                }
            }
            str if str.starts_with("$ cd") => {
                local_sum += recurse(input, total_sum);
            }
            _ => unreachable!(),
        }
    }
    if total_sum < &mut (100000 as usize) {
        *total_sum += local_sum;
    }
    return local_sum;
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use crate::day07::{part_1, part_2};

    #[test]
    fn part_1_example() {
        assert_eq!(95437, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        print!("{}", output);
        //assert_eq!(1702, output);
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(26, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        print!("{}", output);
        //assert_eq!(3559, output);
    }
}
