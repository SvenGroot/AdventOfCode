# Day 24

[Puzzle description.](https://adventofcode.com/2023/day/24)

Part 1 was annoying, as even when I got a correct algorithm, it still didn't work due to floating
point precision. I found a decimal crate, but it panicked on multiplying some of the values. So I
ended up using `f128`, which requires nightly Rust at the moment.
