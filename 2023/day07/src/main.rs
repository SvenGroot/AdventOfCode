// https://adventofcode.com/2023/day/7

use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use aoc::{input::AocInput, iterator::IteratorExt};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Order the hands by strength, where place in the order is rank, multiply rank by bid to get the
// total winnings.
fn part1(input: AocInput) -> usize {
    let mut hands = input.parsed::<Hand>().into_vec();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<_> = cards
            .bytes()
            .map(|card| match card {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => 11,
                b'T' => 10,
                _ => card - b'0',
            })
            .collect();

        let bid = bid.parse().unwrap();
        let hand_type = HandType::from_cards(&cards);
        Ok(Self {
            cards,
            bid,
            hand_type,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let result = self.hand_type.cmp(&other.hand_type);
        if result != Ordering::Equal {
            return result;
        }

        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[u8]) -> Self {
        let mut counts: Vec<_> = cards
            .iter()
            .fold(HashMap::<u8, usize>::new(), |mut map, item| {
                map.entry(*item)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                map
            })
            .into_values()
            .collect();

        counts.sort();
        counts.reverse();
        match counts.as_slice() {
            [5, ..] => Self::FiveOfAKind,
            [4, ..] => Self::FourOfAKind,
            [3, 2, ..] => Self::FullHouse,
            [3, ..] => Self::ThreeOfAKind,
            [2, 2, ..] => Self::TwoPair,
            [2, ..] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6440, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
