// https://adventofcode.com/2021/day/21

use std::{
    collections::HashMap,
    ops::{AddAssign, RangeInclusive},
};

use aoc::{
    input::AocInput,
    iterator::{InfiniteRepeat, InfiniteRepeatExt},
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Play using the deterministic dice until someone gets 1000 points.
fn part1(input: AocInput) -> usize {
    let mut game = DiracDiceGame::from_input(input);
    loop {
        if let Some(result) = game.play_round() {
            return result;
        }
    }
}

// In how many universes does the player that wins the most win?
fn part2(input: AocInput) -> usize {
    let game = QuantumDiceGame::from_input(input);
    let outcome = game.play();
    outcome.0.max(outcome.1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn from_input(input: AocInput) -> [Player; 2] {
        let players: Vec<_> = input
            .map(|line| {
                let (_, tail) = line.rsplit_once(' ').unwrap();
                let position: usize = tail.parse().unwrap();
                Player {
                    position: position - 1,
                    score: 0,
                }
            })
            .collect();

        players.try_into().unwrap()
    }

    fn move_pawn(&mut self, roll: usize) -> usize {
        self.position = (self.position + roll) % 10;
        self.score += self.position + 1;
        self.score
    }
}

struct DiracDiceGame {
    players: [Player; 2],
    dice: InfiniteRepeat<RangeInclusive<usize>>,
    rolls: usize,
}

impl DiracDiceGame {
    fn from_input(input: AocInput) -> Self {
        Self {
            players: Player::from_input(input),
            dice: (1..=100).infinite_repeat(),
            rolls: 0,
        }
    }

    fn play_round(&mut self) -> Option<usize> {
        let mut loser = 1;
        let mut score = self.play(0);
        if score < 1000 {
            score = self.play(1);
            loser = 0;
        }

        (score >= 1000).then_some(self.players[loser].score * self.rolls)
    }

    fn play(&mut self, player: usize) -> usize {
        let roll = self.get_roll() + self.get_roll() + self.get_roll();
        let player = &mut self.players[player];
        player.move_pawn(roll)
    }

    fn get_roll(&mut self) -> usize {
        self.rolls += 1;
        self.dice.next().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct QuantumDiceGame {
    players: [Player; 2],
    turn: bool,
}

impl QuantumDiceGame {
    fn from_input(input: AocInput) -> Self {
        Self {
            players: Player::from_input(input),
            turn: false,
        }
    }

    fn play(self) -> Outcome {
        let mut seen = HashMap::new();
        self.play_round(&mut seen)
    }

    fn play_round(self, seen: &mut HashMap<QuantumDiceGame, Outcome>) -> Outcome {
        if let Some(outcome) = seen.get(&self) {
            return *outcome;
        }

        let player: usize = self.turn.into();
        // All the possible rolls with 3 3-sided dice.
        let mut total = Outcome(0, 0);
        for roll1 in 1..=3 {
            for roll2 in 1..=3 {
                for roll3 in 1..=3 {
                    let mut next = self;
                    let roll = roll1 + roll2 + roll3;
                    let score = next.players[player].move_pawn(roll);
                    if score >= 21 {
                        if self.turn {
                            total.1 += 1;
                        } else {
                            total.0 += 1;
                        }
                    } else {
                        next.turn = !next.turn;
                        total += next.play_round(seen);
                    }
                }
            }
        }

        assert!(seen.insert(self, total).is_none());
        total
    }
}

#[derive(Clone, Copy)]
struct Outcome(usize, usize);

impl AddAssign for Outcome {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(739785, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(444356092776315, part2(AocInput::from_sample()));
    }
}
