// https://adventofcode.com/2021/day/24

use std::{fmt::Display, str::FromStr};

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    let program = Alu::load_program(input);
    //let mut input = [1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 7, 8, 9];
    let mut input = [1; 14];
    for _ in 0..1 {
        let mut alu = Alu::default();
        alu.run(program.iter(), input.iter().copied());
        if alu.registers[3] == 0 {
            println!("{input:?}: valid");
        } else {
            println!("{input:?}: invalid {:b}", alu.registers[3]);
        }

        for i in input.iter_mut().rev() {
            *i += 1;
            if *i != 10 {
                break;
            }

            *i = 1;
        }
    }

    0

    // let mut result = 0;
    // for (index, value) in input.iter().rev().enumerate() {
    //     result += *value as usize * 10usize.pow(index as u32)
    // }

    // result
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(Default)]
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

enum Instruction {
    Input(Register),
    Operation(Operation, Register, Operand),
}

impl Instruction {
    fn execute(&self, alu: &mut Alu, input: &mut impl Iterator<Item = isize>) {
        match self {
            Instruction::Input(reg) => {
                let value = input.next().unwrap();
                println!("inp {reg} [{value}]");
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

                println!("{op} {reg} {operand} [{result} <- {value1} {value2}]");

                reg.set(result, alu);
            }
        }
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
    fn test_part1() {
        assert_eq!(0, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }

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
