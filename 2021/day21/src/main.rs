// https://adventofcode.com/2021/day/21

use std::ops::RangeInclusive;

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

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(Debug)]
struct Player {
    position: usize,
    score: usize,
}

struct DiracDiceGame {
    players: [Player; 2],
    dice: InfiniteRepeat<RangeInclusive<usize>>,
    rolls: usize,
}

impl DiracDiceGame {
    fn from_input(input: AocInput) -> Self {
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

        Self {
            players: players.try_into().unwrap(),
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
        player.position = (player.position + roll) % 10;
        player.score += player.position + 1;
        player.score
    }

    fn get_roll(&mut self) -> usize {
        self.rolls += 1;
        self.dice.next().unwrap()
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
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
