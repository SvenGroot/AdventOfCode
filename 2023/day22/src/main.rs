// https://adventofcode.com/2023/day/22

use aoc::{
    grid3d::{Line3D, PointDiff3D},
    input::AocInput,
    iterator::IteratorExt,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// How many bricks can be disintegrated without moving other bricks after letting the bricks fall
// to their final positions.
fn part1(input: AocInput) -> usize {
    let mut stack = BrickStack::from_input(input);
    stack.settle();
    println!("Settled");
    stack.disintegrate_count()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct BrickStack(Vec<Line3D>);

impl BrickStack {
    fn from_input(input: AocInput) -> Self {
        Self(
            input
                .map(|line| Line3D::parse(&line, "~").unwrap())
                .collect(),
        )
    }

    fn settle(&mut self) {
        loop {
            let mut changed_any = false;
            for i in 0..self.0.len() {
                let changed = self.drop_brick(i);
                changed_any |= changed;
            }

            if !changed_any {
                break;
            }
        }
    }

    // Drop one brick as often as possible
    fn drop_brick(&mut self, index: usize) -> bool {
        let mut changed = false;
        loop {
            let brick = self.0[index];
            if brick.from().z() == 1 || brick.to().z() == 1 {
                return changed;
            }

            let new_brick = brick.add_diff(PointDiff3D::OUT).unwrap(); // Negative Z is down in this grid
            if self
                .0
                .iter()
                .enumerate()
                .any(|(j, b)| index != j && new_brick.intersects(b))
            {
                return changed;
            }

            self.0[index] = new_brick;
            changed = true;
        }
    }

    // Gets the bricks supporting the specified brick.
    fn brick_supports(&self, index: usize) -> Vec<usize> {
        let brick = self.0[index];
        if brick.from().z() == 1 || brick.to().z() == 1 {
            return Vec::new();
        }

        let new_brick = brick.add_diff(PointDiff3D::OUT).unwrap(); // Negative Z is down in this grid
        self.0
            .iter()
            .enumerate()
            .filter_map(|(j, b)| (index != j && new_brick.intersects(b)).then_some(j))
            .collect()
    }

    fn disintegrate_count(&self) -> usize {
        let supports = (0..self.0.len())
            .map(|index| self.brick_supports(index))
            .into_vec();

        // A brick can be disintegrated if there is no brick that has that brick as a sole support.
        (0..self.0.len())
            .filter(|index| !supports.iter().any(|s| s.len() == 1 && s[0] == *index))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
