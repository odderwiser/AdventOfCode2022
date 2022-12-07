mod part1;
mod part2;

fn parse(input: &str) -> Box<dyn Iterator<Item = &str> + '_> {
    Box::new(
        input
            .lines()
            .filter(|x| x.starts_with(|y: char| y.is_numeric() || y == '$')),
    )
}

fn process_folder<'a>(
    input: &mut dyn Iterator<Item = &'a str>,
    local_sum: &mut usize,
) -> Option<&'a str> {
    let mut value = input.next();
    while value.is_some() && !value.unwrap().starts_with('$') {
        // println!("Matching in ls loop: {}", value.unwrap());
        *local_sum += value
            .unwrap()
            .split(' ')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        value = input.next();
    }
    value
}

fn recurse(input: &mut dyn Iterator<Item = &str>, total_repos: &mut Vec<usize>) -> usize {
    let mut local_sum = 0;
    let mut value = input.next();
    while value.is_some() {
        match value.unwrap() {
            "$ cd .." => {
                total_repos.push(local_sum);
                return local_sum;
            }
            "$ ls" => value = process_folder(input, &mut local_sum),
            str if str.starts_with("$ cd") => {
                local_sum += recurse(input, total_repos);
                value = input.next()
            }
            _ => unreachable!(),
        }
    }
    total_repos.push(local_sum);
    return local_sum;
}

#[cfg(test)]
mod test {
    use crate::day07::part1::part_1;
    use crate::day07::part2::part_2;

    #[test]
    fn part_1_example() {
        assert_eq!(95437, part_1(include_str!("test1.txt")));
    }

    #[test]
    fn part_1_puzzle() {
        let output = part_1(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(1315285, output);
    }

    #[test]
    fn part_2_example2() {
        assert_eq!(24933642, part_2(include_str!("test1.txt")));
    }

    #[test]
    fn part_2_puzzle() {
        let output = part_2(include_str!("input.txt"));
        //print!("{}", output);
        assert_eq!(9847279, output);
    }
}
