// https://adventofcode.com/2021/day/17

use aoc::{
    grid::{DiffRectangle, PointDiff},
    input::AocInput,
};
use text_io::scan;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> isize {
    let target = get_target_area(input);
    find_max_y(&target)
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

fn get_target_area(input: AocInput) -> DiffRectangle {
    let min_x: isize;
    let max_x: isize;
    let min_y: isize;
    let max_y: isize;
    let line = input.single_line();
    scan!(line.bytes() => "target area: x={}..{}, y={}..{}", min_x, max_x, min_y, max_y);
    let top_left = PointDiff::new(min_y, min_x);
    let bottom_right = PointDiff::new(max_y, max_x);
    DiffRectangle::new(top_left, bottom_right)
}

#[derive(Debug, PartialEq, Eq)]
enum ShotResult {
    Hit(PointDiff),
    NotHitYet,
    Short,
    Overshot,
}

fn find_max_y(target: &DiffRectangle) -> isize {
    let mut max_y = 0;
    for y in 1..1000 {
        let (result, current_max_y) = find_hit(y, target);
        if matches!(result, ShotResult::Hit(_)) {
            max_y = max_y.max(current_max_y);
        }
    }

    max_y
}

fn find_hit(initial_y: isize, target: &DiffRectangle) -> (ShotResult, isize) {
    for x in 1..=isize::MAX {
        let (result, max_y) = hits_target(PointDiff::new(initial_y, x), target);
        if result != ShotResult::NotHitYet && result != ShotResult::Short {
            return (result, max_y);
        }
    }

    unreachable!();
}

fn hits_target(mut velocity: PointDiff, target: &DiffRectangle) -> (ShotResult, isize) {
    let mut max_y = 0;
    let mut pos = PointDiff::default();
    loop {
        pos += velocity;
        max_y = max_y.max(pos.row());
        let result = check_hit(pos, target);
        if result != ShotResult::NotHitYet {
            return (result, max_y);
        }

        velocity = PointDiff::new(
            velocity.row() - 1,
            if velocity.col() == 0 {
                0
            } else {
                velocity.col() - 1
            },
        )
    }
}

fn check_hit(pos: PointDiff, target: &DiffRectangle) -> ShotResult {
    if target.contains(pos) {
        ShotResult::Hit(pos)
    } else if pos.col() > target.bottom_right().col()
        || pos.col() >= target.top_left().col() && pos.row() < target.bottom_right().row()
    {
        ShotResult::Overshot
    } else if pos.row() < target.bottom_right().row() {
        ShotResult::Short
    } else {
        ShotResult::NotHitYet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(45, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }

    #[test]
    fn test_hit_target() {
        let target = get_target_area(AocInput::from_sample());
        let (result, max_y) = hits_target(PointDiff::new(2, 7), &target);
        assert_eq!(ShotResult::Hit(PointDiff::new(-7, 28)), result);
        assert_eq!(3, max_y);
    }
}
