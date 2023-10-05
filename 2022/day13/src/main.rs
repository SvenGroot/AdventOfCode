use std::{cmp::Ordering, path::Path};

use aoc::{
    aoc_input, get_input, get_input_vec,
    nested_list::{Item, NestedList},
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    get_input_vec(path)
        .split(|line| line.is_empty())
        .enumerate()
        .filter_map(|(index, pair)| {
            assert_eq!(2, pair.len());

            let left: NestedList<u32> = pair[0].parse().expect(&pair[0]);
            let right: NestedList<u32> = pair[1].parse().expect(&pair[1]);

            (cmp_list(&left, &right) == Ordering::Less).then_some(index + 1)
        })
        .sum()
}

fn part2(path: impl AsRef<Path>) -> usize {
    let mut packets: Vec<_> = get_input(path)
        .filter_map(|line| {
            (!line.is_empty()).then(|| {
                let list: NestedList<u32> = line.parse().expect(&line);
                MarkedList::new(list, false)
            })
        })
        .collect();

    packets.push(MarkedList::new("[[2]]".parse().unwrap(), true));
    packets.push(MarkedList::new("[[6]]".parse().unwrap(), true));
    packets.sort_by(|a, b| cmp_list(&a.list, &b.list));
    packets
        .iter()
        .enumerate()
        .filter_map(|(index, p)| p.is_divider.then_some(index + 1))
        .product()
}

struct MarkedList {
    list: NestedList<u32>,
    is_divider: bool,
}

impl MarkedList {
    fn new(list: NestedList<u32>, is_divider: bool) -> Self {
        Self { list, is_divider }
    }
}

fn cmp_list(left: &NestedList<u32>, right: &NestedList<u32>) -> Ordering {
    cmp_slice(left.items(), right.items())
}

fn cmp_slice(left: &[Item<u32>], right: &[Item<u32>]) -> Ordering {
    for (l, r) in left.iter().zip(right) {
        match cmp_item(l, r) {
            Ordering::Equal => (),
            result => return result,
        }
    }

    left.len().cmp(&right.len())
}

fn cmp_item(left: &Item<u32>, right: &Item<u32>) -> Ordering {
    match (left, right) {
        (Item::Value(l), Item::Value(r)) => l.cmp(r),
        (Item::List(l), Item::List(r)) => cmp_slice(l, r),
        (Item::List(l), Item::Value(r)) => cmp_slice(l, &[Item::Value(*r)]),
        (Item::Value(l), Item::List(r)) => cmp_slice(&[Item::Value(*l)], r),
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(140, part2(aoc_sample_input()));
    }
}
