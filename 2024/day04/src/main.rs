// https://adventofcode.com/2024/day/4

use std::char;

use aoc::{
    grid::{Grid, GridBuilder, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find all occurrences of XMAS in the grid, in all directions.
fn part1(input: AocInput) -> usize {
    let search = WordSearch::from_input(input);
    search.find_word("XMAS")
}

fn part2(input: AocInput) -> usize {
    let search = WordSearch::from_input(input);
    search.find_x_shaped(&['M', 'A', 'S'])
}

struct WordSearch(Grid<char>);

impl WordSearch {
    fn from_input(input: AocInput) -> Self {
        Self(GridBuilder::from_input(input).chars().build())
    }

    fn find_word(&self, word: &str) -> usize {
        self.0
            .bounding_rect()
            .edge_points()
            .map(|pos| {
                self.0
                    .get_edge(pos)
                    .get_away_directions()
                    .unwrap()
                    .iter()
                    .map(|&dir| {
                        let Some(_) = self.0.add_point(pos, dir) else {
                            return 0;
                        };

                        let line: String = self.0.scan(pos, dir).collect();
                        find_occurrences(&line, word)
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn find_x_shaped(&self, word: &[char; 3]) -> usize {
        self.0
            .cells()
            .filter(|&(pos, ch)| {
                if *ch != word[1] {
                    return false;
                }

                self.check_dir(pos, PointDiff::UP_LEFT, word).is_some()
                    && self.check_dir(pos, PointDiff::DOWN_LEFT, word).is_some()
            })
            .count()
    }

    fn check_dir(&self, pos: Point, dir: PointDiff, word: &[char; 3]) -> Option<()> {
        let ch1 = self.0[self.0.add_point(pos, dir)?];
        let ch2 = self.0[self.0.add_point(pos, -dir)?];
        (ch1 == word[0] && ch2 == word[2] || ch1 == word[2] && ch2 == word[0]).then_some(())
    }
}

fn find_occurrences(mut input: &str, pattern: &str) -> usize {
    let mut count = 0;
    while let Some(index) = input.find(pattern) {
        count += 1;
        input = &input[(index + 1)..];
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(AocInput::from_sample()));
    }
}
