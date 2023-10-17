// https://adventofcode.com/2021/day/18

use std::{iter::Sum, ops::Add, str::FromStr};

use aoc::{
    input::AocInput,
    nested_list::{Item, NestedList},
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Sum all the numbers, and return the magnitude of the result.
fn part1(input: AocInput) -> usize {
    let sum: SnailfishNumber = input.parsed::<SnailfishNumber>().sum();
    println!("{}", sum.0);
    sum.magnitude()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(Debug, PartialEq, Eq)]
struct SnailfishNumber(NestedList<u8>);

impl SnailfishNumber {
    fn try_explode<'a>(&'a mut self) -> bool {
        let mut state = ExplodeState::<'a> {
            left_number: None,
            right_number: None,
            exploded: false,
        };

        Self::try_explode_core(&mut self.0, &mut state, 0);
        state.exploded
    }

    fn reduce(&mut self) {
        while self.try_explode() || self.try_split() {
            // Nothing
        }
    }

    fn try_split(&mut self) -> bool {
        Self::try_split_core(&mut self.0).is_none()
    }

    fn magnitude(&self) -> usize {
        Self::magnitude_core(&self.0)
    }

    fn try_explode_core<'a>(
        list: &'a mut [Item<u8>],
        state: &mut ExplodeState<'a>,
        level: usize,
    ) -> Option<()> {
        let (left, right) = list.split_at_mut(1);
        Self::try_explode_item(&mut left[0], state, level + 1)?;
        Self::try_explode_item(&mut right[0], state, level + 1)?;
        Some(())
    }

    fn try_explode_item<'a>(
        item: &'a mut Item<u8>,
        state: &mut ExplodeState<'a>,
        level: usize,
    ) -> Option<()> {
        if state.right_number.is_none() && level == 4 {
            if let Item::List(nested) = item {
                if let Some(left_number) = state.left_number.take() {
                    *left_number += nested[0].value().unwrap();
                }

                state.right_number = Some(*nested[1].value().unwrap());
                state.exploded = true;
                *item = Item::Value(0);
                return Some(());
            }
        }

        match item {
            Item::Value(val) => {
                if let Some(num) = state.right_number {
                    *val += num;
                    return None;
                }

                state.left_number = Some(val);
                Some(())
            }
            Item::List(nested) => Self::try_explode_core(nested, state, level),
        }
    }

    fn try_split_core(list: &mut [Item<u8>]) -> Option<()> {
        for item in list {
            match item {
                Item::Value(x) => {
                    if *x >= 10 {
                        let div = *x / 2;
                        *item = Item::List(vec![Item::Value(div), Item::Value(*x - div)]);
                        return None;
                    }
                }
                Item::List(nested) => {
                    Self::try_split_core(nested)?;
                }
            }
        }

        Some(())
    }

    fn magnitude_core(list: &[Item<u8>]) -> usize {
        3 * Self::magnitude_item(&list[0]) + 2 * Self::magnitude_item(&list[1])
    }

    fn magnitude_item(item: &Item<u8>) -> usize {
        match item {
            Item::Value(val) => *val as usize,
            Item::List(nested) => Self::magnitude_core(nested),
        }
    }
}

impl FromStr for SnailfishNumber {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = SnailfishNumber(self.0.combine(rhs.0));
        result.reduce();
        result
    }
}

impl Sum for SnailfishNumber {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first = iter.next().unwrap();
        iter.fold(first, |current, val| current + val)
    }
}

struct ExplodeState<'a> {
    left_number: Option<&'a mut u8>,
    right_number: Option<u8>,
    exploded: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(4140, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }

    #[test]
    fn test_explode() {
        test_explode_helper("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode_helper("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_explode_helper("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_explode_helper(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test_explode_helper(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
        assert!(!SnailfishNumber::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
            .unwrap()
            .try_explode())
    }

    #[test]
    fn test_split() {
        test_split_helper("[10,0]", "[[5,5],0]");
        test_split_helper("[11,20]", "[[5,6],20]");
        assert!(!SnailfishNumber::from_str("[9,0]").unwrap().try_split());
    }

    #[test]
    fn test_add() {
        let left = SnailfishNumber::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let right = SnailfishNumber::from_str("[1,1]").unwrap();
        let result = left + right;
        assert_eq!(
            SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap(),
            result
        );
    }

    fn test_explode_helper(number: &str, expected: &str) {
        let mut number = SnailfishNumber::from_str(number).unwrap();
        assert!(number.try_explode());
        assert_eq!(SnailfishNumber::from_str(expected).unwrap(), number);
    }

    fn test_split_helper(number: &str, expected: &str) {
        let mut number = SnailfishNumber::from_str(number).unwrap();
        assert!(number.try_split());
        assert_eq!(SnailfishNumber::from_str(expected).unwrap(), number);
    }
}
