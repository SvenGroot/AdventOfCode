// https://adventofcode.com/2021/day/23

use std::fmt::Display;

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Get the minimal energy needed to organize the amphipods.
fn part1(input: AocInput) -> usize {
    let burrow = Burrow::<8>::from_input(input);
    burrow.do_move()
}

// Same, with some extra amphipods added in.
// WARNING: Slow in debug mode.
fn part2(input: AocInput) -> usize {
    let burrow = Burrow::<16>::from_input(input);
    burrow.do_move()
}

#[derive(Clone)]
struct Burrow<const C: usize> {
    amphipods: [Amphipod; C],
    room_size: u8,
}

impl<const C: usize> Burrow<C> {
    fn from_input(mut input: AocInput) -> Self {
        let mut amphipods = Vec::new();
        input.next().unwrap();
        input.next().unwrap();
        let line = input.next().unwrap();
        for i in 0..4 {
            let amphipod = line.as_bytes()[3 + 2 * i];
            amphipods.push(Amphipod {
                pos: AmphipodPos::Room(i as u8, 0),
                kind: amphipod as char,
            });
        }

        let line = input.next().unwrap();
        let subroom = if C == 16 { 3 } else { 1 };
        for i in 0..4 {
            let amphipod = line.as_bytes()[3 + 2 * i];
            amphipods.push(Amphipod {
                pos: AmphipodPos::Room(i as u8, subroom),
                kind: amphipod as char,
            });
        }

        // Add in the extra amphipods (this is the same for the sample and real input)
        if C == 16 {
            amphipods.extend_from_slice(&[
                Amphipod {
                    pos: AmphipodPos::Room(0, 1),
                    kind: 'D',
                },
                Amphipod {
                    pos: AmphipodPos::Room(0, 2),
                    kind: 'D',
                },
                Amphipod {
                    pos: AmphipodPos::Room(1, 1),
                    kind: 'C',
                },
                Amphipod {
                    pos: AmphipodPos::Room(1, 2),
                    kind: 'B',
                },
                Amphipod {
                    pos: AmphipodPos::Room(2, 1),
                    kind: 'B',
                },
                Amphipod {
                    pos: AmphipodPos::Room(2, 2),
                    kind: 'A',
                },
                Amphipod {
                    pos: AmphipodPos::Room(3, 1),
                    kind: 'A',
                },
                Amphipod {
                    pos: AmphipodPos::Room(3, 2),
                    kind: 'C',
                },
            ]);
        }

        Self {
            amphipods: amphipods.try_into().unwrap(),
            room_size: (C / 4) as u8,
        }
    }

    fn get_room_char(&self, room: u8, pos: u8) -> char {
        self.amphipods
            .iter()
            .find(|a| a.pos == AmphipodPos::Room(room, pos))
            .map(|a| a.kind)
            .unwrap_or('.')
    }

    fn get_hallway_char(&self, pos: u8) -> char {
        self.amphipods
            .iter()
            .find(|a| a.pos == AmphipodPos::Hallway(pos))
            .map(|a| a.kind)
            .unwrap_or('.')
    }

    fn do_move(self) -> usize {
        // println!("Move:");
        // println!("{self}");
        let mut min_energy: usize = usize::MAX;
        let mut has_move = false;
        let mut final_count = 0;
        // Check if any amphipod can move to its final destination. If so, that is the move we do.
        for (index, a) in self.amphipods.iter().enumerate() {
            if a.current_room() != Some(a.target_room()) {
                if let Some(pos) = self.room_available(a.target_room()) {
                    has_move |= self.do_next_move(pos, index, &mut min_energy);
                }
            } else if a.is_final(&self) {
                final_count += 1;
            }
        }

        // If we did a move to the correct room, no need to try anything else.
        if has_move {
            return min_energy;
        } else if final_count == self.amphipods.len() {
            return 0;
        }

        // Otherwise, try all moves where an arthropod moves into the hallway.
        for (index, a) in self
            .amphipods
            .iter()
            .enumerate()
            .filter(|(_, a)| !a.is_final(&self))
        {
            if let AmphipodPos::Room(room, pos) = a.pos {
                if pos == 1 && !self.pos_free(&AmphipodPos::Room(room, 0)) {
                    continue;
                }

                for i in [0, 1, 3, 5, 7, 9, 10] {
                    let dest = AmphipodPos::Hallway(i);
                    if self.pos_free(&dest) {
                        self.do_next_move(dest, index, &mut min_energy);
                    }
                }
            }
        }

        min_energy
    }

    fn do_next_move(&self, pos: AmphipodPos, index: usize, min_energy: &mut usize) -> bool {
        let a = &self.amphipods[index];
        if let Some(energy) = self.path_free(&a.pos, &pos) {
            let mut next = self.clone();
            next.amphipods[index].pos = pos;
            let move_energy = a.energy_use() * energy;
            // println!("{move_energy}");
            let next_energy = next.do_move();
            if next_energy != usize::MAX {
                *min_energy = (*min_energy).min(move_energy + next_energy);
            }

            true
        } else {
            false
        }
    }

    fn room_available(&self, room: u8) -> Option<AmphipodPos> {
        for subroom in (0..self.room_size).rev() {
            if let Some(a) = self.get_pos(&AmphipodPos::Room(room, subroom)) {
                if a.target_room() != room {
                    break;
                }
            } else {
                return Some(AmphipodPos::Room(room, subroom));
            }
        }

        None
    }

    fn pos_free(&self, pos: &AmphipodPos) -> bool {
        self.amphipods.iter().all(|a| a.pos != *pos)
    }

    fn get_pos(&self, pos: &AmphipodPos) -> Option<&Amphipod> {
        self.amphipods.iter().find(|a| a.pos == *pos)
    }

    fn path_free(&self, from: &AmphipodPos, to: &AmphipodPos) -> Option<usize> {
        let (from, from_energy) = self.pos_to_hallway_index(from)?;
        let (to, to_energy) = self.pos_to_hallway_index(to)?;

        let source = from.min(to);
        let dest = from.max(to);
        for pos in (source + 1)..dest {
            if !self.pos_free(&AmphipodPos::Hallway(pos)) {
                return None;
            }
        }

        let energy = dest - source + from_energy + to_energy;
        Some(energy as usize)
    }

    fn pos_to_hallway_index(&self, from: &AmphipodPos) -> Option<(u8, u8)> {
        let mut energy = 0;
        let hallway_pos = match from {
            AmphipodPos::Room(room, pos) => {
                energy += 1;
                if *pos != 0 {
                    energy += pos;
                    if (0..*pos).any(|subroom| !self.pos_free(&AmphipodPos::Room(*room, subroom))) {
                        return None;
                    }
                }

                room * 2 + 2
            }
            AmphipodPos::Hallway(pos) => *pos,
        };

        Some((hallway_pos, energy))
    }

    const HALLWAY_LEN: u8 = 11;
}

impl<const C: usize> Display for Burrow<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for pos in 0..Self::HALLWAY_LEN {
            write!(f, "{}", self.get_hallway_char(pos))?;
        }

        writeln!(f, "#")?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.get_room_char(0, 0),
            self.get_room_char(1, 0),
            self.get_room_char(2, 0),
            self.get_room_char(3, 0),
        )?;

        for subroom in 1..self.room_size {
            writeln!(
                f,
                "  #{}#{}#{}#{}#",
                self.get_room_char(0, subroom),
                self.get_room_char(1, subroom),
                self.get_room_char(2, subroom),
                self.get_room_char(3, subroom),
            )?;
        }

        writeln!(f, "  #########")
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Amphipod {
    pos: AmphipodPos,
    kind: char,
}

impl Amphipod {
    fn target_room(&self) -> u8 {
        self.kind as u8 - b'A'
    }

    fn current_room(&self) -> Option<u8> {
        match &self.pos {
            AmphipodPos::Room(room, _) => Some(*room),
            AmphipodPos::Hallway(_) => None,
        }
    }

    fn is_final<const C: usize>(&self, burrow: &Burrow<C>) -> bool {
        if let AmphipodPos::Room(room, pos) = self.pos {
            room == self.target_room()
                && (pos == burrow.room_size - 1
                    || (pos + 1..burrow.room_size).all(|subroom| {
                        burrow
                            .get_pos(&AmphipodPos::Room(room, subroom))
                            .unwrap()
                            .target_room()
                            == room
                    }))
        } else {
            false
        }
    }

    fn energy_use(&self) -> usize {
        match self.kind {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AmphipodPos {
    Hallway(u8),
    Room(u8, u8),
}

impl Default for AmphipodPos {
    fn default() -> Self {
        AmphipodPos::Hallway(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(12521, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        // WARNING: Slow in debug mode.
        assert_eq!(44169, part2(AocInput::from_sample()));
    }
}
