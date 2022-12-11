use aoc::get_input;

fn main() {
    const PATH: &str = "input/day10.txt";
    let (signal, display) = run_program(PATH);
    println!("Part 1: {}", signal);
    println!("Part 2:\n{}", display);
}

fn run_program(path: &str) -> (i32, String) {
    let mut cpu = Cpu::new();
    let signal = get_input(path)
        .map(|line| {
            match line
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .as_slice()
            {
                ["noop"] => cpu.run(1, 0),
                ["addx", value] => cpu.run(2, value.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .sum();

    (signal, cpu.display)
}

struct Cpu {
    cycle: usize,
    x: i32,
    display: String,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            cycle: 0,
            x: 1,
            display: String::new(),
        }
    }

    fn run(&mut self, cycles: usize, add_x: i32) -> i32 {
        let result = (0..cycles).map(|_| self.cycle()).sum();
        self.x += add_x;
        result
    }

    fn cycle(&mut self) -> i32 {
        self.cycle += 1;
        self.draw();
        if self.cycle >= 20 && self.cycle <= 220 && (self.cycle - 20) % 40 == 0 {
            (self.cycle as isize * self.x as isize) as i32
        } else {
            0
        }
    }

    fn draw(&mut self) {
        let pos = ((self.cycle - 1) % 40) as i32;
        let ch = if (self.x - pos).abs() <= 1 { '#' } else { '.' };

        self.display.push(ch);
        if pos == 39 {
            self.display.push('\n');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "../input/sample/day10.txt";
    const EXPECTED: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn test_parts() {
        let (signal, display) = run_program(PATH);
        assert_eq!(13140, signal);
        println!("{}", display);
        assert_eq!(EXPECTED, display);
    }
}
