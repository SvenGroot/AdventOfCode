// https://adventofcode.com/2021/day/24

use std::{fmt::Display, str::FromStr};

use aoc::{input::AocInput, iterator::IteratorExt, slice::SliceExt};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find the largest accepted code.
fn part1(input: AocInput) -> isize {
    find_code(input, false)
}

// Find the smallest accepted code.
fn part2(input: AocInput) -> isize {
    find_code(input, true)
}

fn find_code(input: AocInput, smallest: bool) -> isize {
    let program = Alu::load_program(input);

    // The program is a bunch of repeated blocks, that differ only by a few parameters.
    // The blocks fall into two kinds:
    // 1. z = (z * 26) + digit + number
    //    This is unconditional because the test value is always too high to match the digit. The
    //    number differs per block.
    // 2. if digit == (z % 26) - number { z /= 26 } else { z += w + number }
    //    We never want to take the else path.
    let blocks = program
        .split_inclusive_start(Instruction::is_input)
        .into_vec();

    // Each kind 1 block pairs up with a kind 2 block that undoes it (they are equal in count).
    // Find the matching pairs.
    let mut pairs = Vec::new();
    let mut stack = Vec::new();
    for (index, block) in blocks.iter().enumerate() {
        if block.contains(&Instruction::Operation(
            Operation::Divide,
            Register(3),
            Operand::Value(26),
        )) {
            let pair_index = stack.pop().unwrap();
            pairs.push((pair_index, index));
        } else {
            stack.push(index);
        }
    }

    // We can consider each pair in isolation. Z must be unchanged after running both blocks in the
    // pair. Find either the highest or lowest set of digits that achieve that.
    let mut digits = [0; 14];
    for (first, second) in pairs.iter() {
        let mut input = if smallest { [1, 1] } else { [9, 9] };
        loop {
            let mut input_iter = input.iter().copied();
            let mut alu = Alu::default();
            alu.run(blocks[*first].iter(), &mut input_iter);
            alu.run(blocks[*second].iter(), &mut input_iter);

            if alu.registers[3] == 0 {
                digits[*first] = input[0];
                digits[*second] = input[1];
                break;
            }

            if smallest {
                assert_ne!([9, 9], input);
                for i in input.iter_mut().rev() {
                    *i += 1;
                    if *i != 10 {
                        break;
                    }

                    *i = 1;
                }
            } else {
                assert_ne!([1, 1], input);
                for i in input.iter_mut().rev() {
                    *i -= 1;
                    if *i != 0 {
                        break;
                    }

                    *i = 9;
                }
            }
        }
    }

    // Convert the digits array to a number.
    digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |current, (index, new)| {
            current + *new * 10isize.pow(index as u32)
        })
}

#[derive(Default, Clone)]
struct Alu {
    registers: [isize; 4],
}

impl Alu {
    fn load_program(program: impl Iterator<Item = String>) -> Vec<Instruction> {
        program.map(|line| line.parse().unwrap()).collect()
    }

    fn run<'a>(
        &mut self,
        program: impl Iterator<Item = &'a Instruction>,
        mut input: impl Iterator<Item = isize>,
    ) {
        for instruction in program {
            instruction.execute(self, &mut input);
        }
    }
}

#[derive(PartialEq, Eq)]
enum Instruction {
    Input(Register),
    Operation(Operation, Register, Operand),
}

impl Instruction {
    fn execute(&self, alu: &mut Alu, input: &mut impl Iterator<Item = isize>) {
        match self {
            Instruction::Input(reg) => {
                let value = input.next().unwrap();
                reg.set(value, alu);
            }
            Instruction::Operation(op, reg, operand) => {
                let value1 = reg.get(alu);
                let value2 = operand.get(alu);
                let result = match op {
                    Operation::Add => value1 + value2,
                    Operation::Multiply => value1 * value2,
                    Operation::Divide => value1 / value2,
                    Operation::Modulo => value1 % value2,
                    Operation::Equals => (value1 == value2).into(),
                };

                reg.set(result, alu);
            }
        }
    }

    fn is_input(&self) -> bool {
        matches!(self, Instruction::Input(_))
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let opcode = parts.next().unwrap();
        let register = parts.next().unwrap().parse().unwrap();
        if opcode == "inp" {
            return Ok(Instruction::Input(register));
        }

        let operand = parts.next().unwrap().parse().unwrap();
        let operation = match opcode {
            "add" => Operation::Add,
            "mul" => Operation::Multiply,
            "div" => Operation::Divide,
            "mod" => Operation::Modulo,
            "eql" => Operation::Equals,
            _ => unreachable!(),
        };

        Ok(Instruction::Operation(operation, register, operand))
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Input(reg) => write!(f, "inp {reg}"),
            Instruction::Operation(op, reg, operand) => write!(f, "{op} {reg} {operand}"),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Register(usize);

impl Register {
    fn set(&self, value: isize, alu: &mut Alu) {
        alu.registers[self.0] = value;
    }

    fn get(&self, alu: &Alu) -> isize {
        alu.registers[self.0]
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(1, s.len());
        Ok(Self((s.as_bytes()[0] - b'w') as usize))
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 as u8 + b'w') as char)
    }
}

#[derive(PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Divide,
    Modulo,
    Equals,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Operation::Add => "add",
            Operation::Multiply => "mul",
            Operation::Divide => "div",
            Operation::Modulo => "mod",
            Operation::Equals => "eql",
        };

        write!(f, "{text}")
    }
}

#[derive(PartialEq, Eq)]
enum Operand {
    Register(Register),
    Value(isize),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(reg) => write!(f, "{reg}"),
            Operand::Value(val) => write!(f, "{val}"),
        }
    }
}

impl Operand {
    fn get(&self, alu: &Alu) -> isize {
        match self {
            Operand::Register(reg) => reg.get(alu),
            Operand::Value(value) => *value,
        }
    }
}

impl FromStr for Operand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[0].is_ascii_alphabetic() {
            Ok(Operand::Register(s.parse().unwrap()))
        } else {
            Ok(Operand::Value(s.parse().unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu() {
        let input = AocInput::from_sample().into_vec();
        let mut input = input.split(|line| line.is_empty());

        let program = Alu::load_program(input.next().unwrap().iter().cloned());
        let mut alu = Alu::default();
        alu.run(program.iter(), [13].into_iter());
        assert_eq!([0, -13, 0, 0], alu.registers);

        let program = Alu::load_program(input.next().unwrap().iter().cloned());
        let mut alu = Alu::default();
        alu.run(program.iter(), [3, 9].into_iter());
        assert_eq!(1, alu.registers[3]);
        let mut alu = Alu::default();
        alu.run(program.iter(), [3, 8].into_iter());
        assert_eq!(0, alu.registers[3]);

        let program = Alu::load_program(input.next().unwrap().iter().cloned());
        let mut alu = Alu::default();
        alu.run(program.iter(), [13].into_iter());
        assert_eq!([1, 1, 0, 1], alu.registers);
    }
}
