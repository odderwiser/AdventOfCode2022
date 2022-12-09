use crate::day08::{compute, Tree};

pub fn part_2(input: &str) -> usize {
    compute(input, traverse)
        .iter()
        .flatten()
        .map(|x| x.scenic_factor)
        .max()
        .unwrap() as usize
}

//you see all trees until the next tree of the same height as you
fn traverse(array: Box<&mut dyn Iterator<Item = &mut Vec<Tree>>>, rev: bool) {
    let mut current_height: Vec<i8> = Vec::new();
    array.for_each(|x| {
        current_height = Vec::new();
        let mut it: Box<dyn DoubleEndedIterator<Item = &mut Tree>> = Box::new(x.iter_mut());
        if rev {
            it = Box::new(it.rev());
        }
        it.for_each(|y: &mut Tree| {
            let see_trees = match current_height.iter().rposition(|x| x >= &y.height) {
                None => current_height.len(),
                Some(val) => current_height.len() - val,
            };
            y.scenic_factor(see_trees as u32);
            current_height.push(y.height);
        })
    });
}
