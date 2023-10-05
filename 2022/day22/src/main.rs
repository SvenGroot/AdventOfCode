// https://adventofcode.com/2022/day/22
use std::{fmt::Display, num::NonZeroUsize, path::Path, str::FromStr};

use aoc::{
    aoc_input, get_input, get_input_vec,
    grid::{Grid, GridBuilder, Point, PointDiff, Rectangle, Rotation},
    iterator::PeekableExt,
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

// Walk the grid using the path given, stopping at walls, wrapping to the other side if you walk off
// the board.
fn part1(path: impl AsRef<Path>) -> usize {
    let (mut board, moves) = load_input(path);
    for mv in moves {
        board.do_move(&mv);
    }

    println!("{}", board.grid);
    board.get_password()
}

// Same as part1, but fold the board into a cube, and continue on the adjacent face when you walk
// off the board.
fn part2(path: impl AsRef<Path>) -> usize {
    let path = path.as_ref();
    let (mut board, moves) = load_input(path);

    // Extra file containing cube faces and face transitions; not part of original input. See
    // readme.
    let cube_path = path.with_file_name("day22_cube.txt");
    board.cube = Some(Cube::from_file(cube_path));
    for mv in moves {
        board.do_move(&mv);
    }

    println!("{}", board.grid);
    board.get_password()
}

fn load_input(path: impl AsRef<Path>) -> (Board, Vec<Move>) {
    let mut input = get_input(path);
    let grid = input.by_ref().take_while(|line| !line.is_empty());
    let mut grid = GridBuilder::from_lines(grid)
        .map(|ch| match ch {
            b' ' => Tile::Blank,
            b'.' => Tile::Open(None),
            b'#' => Tile::Wall,
            _ => unreachable!(),
        })
        .extend(0, 0, b' ')
        .build();

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
            cube: None,
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
    cube: Option<Cube>,
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
            let (new_pos, new_dir) = match new_pos {
                None => self.wrap_pos(),
                Some(pt) if matches!(self.grid[pt], Tile::Blank) => self.wrap_pos(),
                Some(pt) => (pt, self.current_dir),
            };

            if matches!(self.grid[new_pos], Tile::Wall) {
                break;
            }

            self.current_pos = new_pos;
            self.current_dir = new_dir;
            self.grid[new_pos] = Tile::Open(Some(self.current_dir));
        }
    }

    fn get_password(&self) -> usize {
        let facing = match self.current_dir {
            PointDiff::RIGHT => 0,
            PointDiff::DOWN => 1,
            PointDiff::LEFT => 2,
            PointDiff::UP => 3,
            _ => unreachable!(),
        };

        1000 * (self.current_pos.row() + 1) + 4 * (self.current_pos.col() + 1) + facing
    }

    fn wrap_pos(&self) -> (Point, PointDiff) {
        if let Some(cube) = &self.cube {
            return cube.do_transition(self.current_pos, self.current_dir);
        }

        let predicate = |tile: &Tile| !matches!(tile, Tile::Blank);

        let new_pos = match self.current_dir {
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
        };

        (new_pos, self.current_dir)
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

#[derive(Debug, Clone)]
struct FaceTransition {
    from_face: usize,
    dir: PointDiff,
    new_face: usize,
    new_dir: PointDiff,
    reverse: bool,
}

impl FaceTransition {
    fn invert(&self) -> Self {
        Self {
            from_face: self.new_face,
            dir: self.new_dir.invert(),
            new_face: self.from_face,
            new_dir: self.dir.invert(),
            reverse: self.reverse,
        }
    }
}

impl FromStr for FaceTransition {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split(',').collect();
        let bytes = splits[0].as_bytes();
        let from_face = bytes[0] - b'1';
        let dir = match bytes[1] {
            b'T' => PointDiff::UP,
            b'R' => PointDiff::RIGHT,
            b'B' => PointDiff::DOWN,
            b'L' => PointDiff::LEFT,
            _ => unreachable!(),
        };

        let bytes = splits[1].as_bytes();
        let new_face = bytes[0] - b'1';
        let new_dir = match bytes[1] {
            b'T' => PointDiff::DOWN,
            b'R' => PointDiff::LEFT,
            b'B' => PointDiff::UP,
            b'L' => PointDiff::RIGHT,
            _ => unreachable!(),
        };

        let reverse = splits[2] == "true";
        Ok(Self {
            from_face: from_face.into(),
            dir,
            new_face: new_face.into(),
            new_dir,
            reverse,
        })
    }
}

#[derive(Debug)]
struct Cube {
    faces: [Rectangle; 6],
    transitions: Vec<FaceTransition>,
}

impl Cube {
    fn from_file(path: impl AsRef<Path>) -> Self {
        let input = get_input_vec(path);
        let face_size = input[0].parse().unwrap();
        let layout: Vec<_> = input[1]
            .split(';')
            .map(|pos| Point::from_str(pos).unwrap())
            .collect();

        let faces = Cube::from_grid(layout.try_into().unwrap(), face_size);
        let mut transitions: Vec<_> = input[2..]
            .iter()
            .map(|line| FaceTransition::from_str(line).unwrap())
            .collect();

        transitions.extend(transitions.clone().iter().map(FaceTransition::invert));

        Self { faces, transitions }
    }

    fn from_grid(layout: [Point; 6], face_size: NonZeroUsize) -> [Rectangle; 6] {
        layout.map(|face| Rectangle::from_size(face * face_size.get(), face_size, face_size))
    }

    fn do_transition(&self, pos: Point, dir: PointDiff) -> (Point, PointDiff) {
        let from_face = self.get_face(pos);
        let transition = self
            .transitions
            .iter()
            .find(|t| t.from_face == from_face && t.dir == dir)
            .unwrap_or_else(|| panic!("Should have {from_face} and {dir:?} (pos = {pos:?})"));

        let from_face = &self.faces[from_face];
        let offset = pos - from_face.top_left();

        // If we are moving off a side, we need to translate the row to the new face, otherwise the
        // column.
        let source_pos = if Self::is_side(dir) {
            offset.row()
        } else {
            offset.col()
        };

        // If we are entering the new face on a side, the column will be either the left or right
        // edge of the new face (depending on the direction), while the row will use the translated
        // source value. This is done either from the start or end of the edge depending on the
        // reverse value.
        let new_face = &self.faces[transition.new_face];
        let new_pos = if Self::is_side(transition.new_dir) {
            let new_col = if transition.new_dir == PointDiff::RIGHT {
                new_face.top_left().col()
            } else {
                new_face.bottom_right().col()
            };

            let new_row = if transition.reverse {
                new_face.bottom_right().row() - source_pos
            } else {
                new_face.top_left().row() + source_pos
            };

            Point::new(new_row, new_col)
        } else {
            let new_row = if transition.new_dir == PointDiff::DOWN {
                new_face.top_left().row()
            } else {
                new_face.bottom_right().row()
            };

            let new_col = if transition.reverse {
                new_face.bottom_right().col() - source_pos
            } else {
                new_face.top_left().col() + source_pos
            };

            Point::new(new_row, new_col)
        };

        (new_pos, transition.new_dir)
    }

    fn get_face(&self, pos: Point) -> usize {
        self.faces
            .iter()
            .position(|face| face.contains(pos))
            .unwrap_or_else(|| panic!("No face for {pos:?}"))
    }

    fn is_side(dir: PointDiff) -> bool {
        dir == PointDiff::LEFT || dir == PointDiff::RIGHT
    }
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
        assert_eq!(5031, part2(aoc_sample_input()));
    }
}
