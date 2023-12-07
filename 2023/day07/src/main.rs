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
    play_game(input.parsed::<Hand>().into_vec())
}

// The same, but treat J as a joker instead of a jack.
fn part2(input: AocInput) -> usize {
    play_game(input.map(|line| Hand::from_str(&line, true)).into_vec())
}

fn play_game(mut hands: Vec<Hand>) -> usize {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn from_str(s: &str, use_joker: bool) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<_> = cards
            .bytes()
            .map(|card| match card {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => {
                    if use_joker {
                        1
                    } else {
                        11
                    }
                }
                b'T' => 10,
                _ => card - b'0',
            })
            .collect();

        let bid = bid.parse().unwrap();
        let hand_type = HandType::from_cards(&cards, use_joker);
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str(s, false))
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
    fn from_cards(cards: &[u8], use_joker: bool) -> Self {
        let joker_count = if use_joker {
            cards.iter().filter(|&&c| c == 1).count()
        } else {
            0
        };

        if joker_count == 5 {
            return Self::FiveOfAKind;
        }

        let mut counts: Vec<_> = cards
            .iter()
            .filter(|&&c| !use_joker || c != 1)
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
        counts[0] += joker_count;
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
        assert_eq!(5905, part2(AocInput::from_sample()));
    }
}
