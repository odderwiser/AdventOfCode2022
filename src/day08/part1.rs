use crate::day08::{compute, Tree};

pub fn part_1(input: &str) -> usize {
    let arr = compute(input, traverse);
    arr.iter().flatten().filter(|x| x.is_visible()).count()
}

fn traverse(array: &mut dyn Iterator<Item = &mut Vec<Tree>>, rev: bool) {
    let mut current_height: i8 = -1;
    array.for_each(|x| {
        current_height = -1;
        let mut it: Box<dyn DoubleEndedIterator<Item = &mut Tree>> = Box::new(x.iter_mut());
        if rev {
            it = Box::new(it.rev());
        }
        it.for_each(|y: &mut Tree| {
            if y.height > current_height {
                current_height = y.height;
                y.visible();
            }
        })
    });
}
