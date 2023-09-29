// https://adventofcode.com/2022/day/22
use std::{fmt::Display, path::Path};

use aoc::{
    aoc_input, get_input,
    grid::{grid_from_non_uniform_lines, Grid, Point, PointDiff, Rotation},
    iterator::PeekableExt,
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let (mut board, moves) = load_input(path);
    for mv in moves {
        board.do_move(&mv);
    }

    println!("{}", board.grid);
    let facing = match board.current_dir {
        PointDiff::RIGHT => 0,
        PointDiff::DOWN => 1,
        PointDiff::LEFT => 2,
        PointDiff::UP => 3,
        _ => unreachable!(),
    };

    1000 * (board.current_pos.row() + 1) + 4 * (board.current_pos.col() + 1) + facing
}

fn part2(path: impl AsRef<Path>) -> usize {
    get_input(path).map(|_| 0).sum()
}

fn load_input(path: impl AsRef<Path>) -> (Board, Vec<Move>) {
    let mut input = get_input(path);
    let grid = input.by_ref().take_while(|line| !line.is_empty());
    let mut grid = grid_from_non_uniform_lines(grid, b' ').map(|ch| match ch {
        b' ' => Tile::Blank,
        b'.' => Tile::Open(None),
        b'#' => Tile::Wall,
        _ => unreachable!(),
    });

    let moves = input.next().unwrap();
    let mut peekable_chars = moves.chars().peekable();
    let mut moves = Vec::new();
    loop {
        let number: String = peekable_chars
            .take_while_peek(|ch| ch.is_numeric())
            .collect();

        if number.is_empty() {
            break;
        }

        moves.push(Move::Walk(number.parse().unwrap()));
        let dir = match peekable_chars.next() {
            Some('R') => Rotation::Right,
            Some('L') => Rotation::Left,
            None => break,
            Some(ch) => unreachable!("{ch}"),
        };

        moves.push(Move::Turn(dir));
    }

    let start = grid
        .find_in_row(0, |tile| matches!(tile, Tile::Open(_)))
        .unwrap();

    grid[start] = Tile::Open(Some(PointDiff::RIGHT));

    (
        Board {
            grid,
            current_pos: start,
            current_dir: PointDiff::RIGHT,
        },
        moves,
    )
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Blank,
    Open(Option<PointDiff>),
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Blank => ' ',
            Tile::Open(None) => '.',
            Tile::Open(Some(dir)) => dir.get_dir_char().unwrap(),
            Tile::Wall => '#',
        };

        write!(f, "{}", ch)
    }
}

struct Board {
    grid: Grid<Tile>,
    current_pos: Point,
    current_dir: PointDiff,
}

impl Board {
    fn do_move(&mut self, mv: &Move) {
        match mv {
            Move::Walk(count) => self.walk(*count),
            Move::Turn(rotation) => self.turn(*rotation),
        }
    }

    fn walk(&mut self, count: u32) {
        for _ in 0..count {
            let new_pos = self.grid.add_point(self.current_pos, self.current_dir);
            let new_pos = match new_pos {
                None => self.wrap_pos(),
                Some(pt) if matches!(self.grid[pt], Tile::Blank) => self.wrap_pos(),
                Some(pt) => pt,
            };

            if matches!(self.grid[new_pos], Tile::Wall) {
                break;
            }

            self.grid[new_pos] = Tile::Open(Some(self.current_dir));
            self.current_pos = new_pos;
        }
    }

    fn wrap_pos(&self) -> Point {
        let predicate = |tile: &Tile| !matches!(tile, Tile::Blank);

        match self.current_dir {
            PointDiff::RIGHT => self
                .grid
                .find_in_row(self.current_pos.row(), predicate)
                .unwrap(),
            PointDiff::LEFT => self
                .grid
                .rfind_in_row(self.current_pos.row(), predicate)
                .unwrap(),
            PointDiff::UP => self
                .grid
                .rfind_in_col(self.current_pos.col(), predicate)
                .unwrap(),
            PointDiff::DOWN => self
                .grid
                .find_in_col(self.current_pos.col(), predicate)
                .unwrap(),
            _ => unreachable!(),
        }
    }

    fn turn(&mut self, rotation: Rotation) {
        self.current_dir = self.current_dir.rotate(rotation);
        self.grid[self.current_pos] = Tile::Open(Some(self.current_dir));
    }
}

enum Move {
    Walk(u32),
    Turn(Rotation),
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6032, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(aoc_sample_input()));
    }
}
