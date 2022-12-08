fn parse(input: &str) -> Vec<Vec<Tree>> {
    input.lines()
        .map(|x| x.chars().map(|y |Tree::new(y.to_digit(10).unwrap() as i8))
            .collect::<Vec<Tree>>()).collect::<Vec<_>>()
}

fn part_2(input: &str) -> usize {
    let mut array = parse(input);
    traverse_count(Box::new(&mut array.iter_mut()), false);
    traverse_count(Box::new(&mut array.iter_mut()), true);
    let mut arr2 = transpose(array);
    traverse_count(Box::new(&mut arr2.iter_mut()), false);
    traverse_count(Box::new(&mut arr2.iter_mut()), true);
    arr2.iter().flatten().map(|x| x.see_trees).max().unwrap() as usize
}

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

fn traverse(array: Box<&mut dyn Iterator<Item=&mut Vec<Tree>>>, dir:Dir, rev: bool) {
    let mut current_height:i8 = -1;
    array.for_each(|x| {
        current_height = -1;
        let mut it: Box<dyn DoubleEndedIterator<Item=&mut Tree> >=Box::new( x.iter_mut());
        if rev {
            it  = Box::new(it.rev());
        }
            it.for_each(|y: &mut Tree| {
            if y.height > current_height  {
                current_height =y.height;
            } else {
                y.invisible(&dir);
            }
        })
    });
}

//you see all trees until the next tree of the same height as you
fn traverse_count(array: Box<&mut dyn Iterator<Item=&mut Vec<Tree>>>, rev: bool) {
    let mut current_height: Vec<i8> = Vec::new();
    let mut see_trees:u32 = 0;
    array.for_each(|x| {
        current_height = Vec::new();
        see_trees = 0;
        let mut it: Box<dyn DoubleEndedIterator<Item=&mut Tree> >=Box::new( x.iter_mut());
        if rev {
            it  = Box::new(it.rev());
        }
        it.for_each(|y: &mut Tree| {
            let see_trees = match current_height.iter().rposition(|x|x >= &y.height) {
                None => current_height.len(),
                Some(val) => current_height.len()-val
            };
            y.add(see_trees as u32);
            current_height.push(y.height);
        })
    });
}

fn part_1(input: &str) -> usize {
    let mut array = parse(input);
    traverse(Box::new(&mut array.iter_mut()), Dir::LEFT, false);
    traverse(Box::new(&mut array.iter_mut()), Dir::RIGHT, true);
    let mut arr2 = transpose(array);
    traverse(Box::new(&mut arr2.iter_mut()), Dir::TOP, false);
    traverse(Box::new(&mut arr2.iter_mut()), Dir::BOTTOM, true);
    arr2.iter().flatten().filter(|x|x.is_visible()).count()
}

#[derive(Debug)]
struct Tree {
    visible_left: bool,
    visible_right: bool,
    visible_top: bool,
    visible_bottom: bool,
    height: i8,
    see_trees :u32
}

impl Tree {
    fn new(height: i8) -> Tree{
        Tree{
            visible_left: true,
            visible_right: true,
            visible_top: true,
            visible_bottom: true,
            height,
            see_trees: 1
        }
    }

    fn invisible(&mut self, dir: &Dir) {
        match dir {
            Dir::LEFT => self.visible_left = false,
            Dir::RIGHT => self.visible_right = false,
            Dir::TOP => self.visible_top = false,
            Dir::BOTTOM => self.visible_bottom = false
        }
    }

    fn is_visible(&self) -> bool {
        return self.visible_bottom || self.visible_left || self.visible_right || self.visible_top
    }

    fn add(&mut self, trees: u32) {
        self.see_trees*=trees;
    }
}

enum Dir {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM
}


#[cfg(test)]
mod test {
    use crate::day08::{part_1, part_2};

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