// https://adventofcode.com/2024/day/4

use aoc::{
    grid::{Grid, GridBuilder},
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
    input.map(|_| 0).sum()
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
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
