// https://adventofcode.com/2022/day/2

use aoc::get_input;

fn main() {
    let score: i32 = get_input("input/day2.txt")
        .map(|line| {
            let opponent_choice = line.as_bytes()[0] as i32 - 'A' as i32;
            let outcome = line.as_bytes()[2] as i32 - 'Y' as i32;
            let my_choice = (opponent_choice + outcome).rem_euclid(3);
            my_choice
                + 1
                + match outcome {
                    1 => 6,
                    0 => 3,
                    _ => 0,
                }
        })
        .sum();

    println!("Total score: {}", score);
}
