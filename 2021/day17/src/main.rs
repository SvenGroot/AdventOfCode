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
    let target = get_target_area(input);
    get_hit_count(&target)
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

fn find_max_y(target: &DiffRectangle) -> isize {
    let mut max_y = 0;
    // Just pick some random reasonable upper bound for the search to make the code simpler.
    for y in 1..1000 {
        if let Some(current_max_y) = find_hit(y, target) {
            max_y = max_y.max(current_max_y);
        }
    }

    max_y
}

fn get_hit_count(target: &DiffRectangle) -> usize {
    let mut count = 0;
    // Because the rectangle is upside down, top_left is actually the lower y value.
    for y in target.top_left().row()..1000 {
        for x in 1..=target.bottom_right().col() + 1 {
            let velocity = PointDiff::new(y, x);
            if hits_target(velocity, target).is_some() {
                count += 1
            }
        }
    }

    count
}

fn find_hit(initial_y: isize, target: &DiffRectangle) -> Option<isize> {
    for x in 1..=target.bottom_right().col() + 1 {
        if let Some((_, max_y)) = hits_target(PointDiff::new(initial_y, x), target) {
            return Some(max_y);
        }
    }

    None
}

// There's probably math you could use to determine if it hits rather than simulating the
// trajectory, but this works and is fast enough.
fn hits_target(mut velocity: PointDiff, target: &DiffRectangle) -> Option<(PointDiff, isize)> {
    let mut max_y = 0;
    let mut pos = PointDiff::default();
    loop {
        pos += velocity;
        max_y = max_y.max(pos.row());
        if target.contains(pos) {
            return Some((pos, max_y));
        }

        if pos.row() < target.top_left().row() || pos.col() > target.bottom_right().col() {
            // Overshot either in x or y direction; will not hit.
            return None;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(45, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(112, part2(AocInput::from_sample()));
    }

    #[test]
    fn test_hit_target() {
        let target = get_target_area(AocInput::from_sample());
        let (result, max_y) = hits_target(PointDiff::new(2, 7), &target).unwrap();
        assert_eq!(PointDiff::new(-7, 28), result);
        assert_eq!(3, max_y);
    }
}
