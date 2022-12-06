use std::fmt::Display;

pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    pub fn new(input: &[String]) -> Stacks {
        let mut stacks = Vec::new();
        for line in input.iter().rev() {
            if stacks.is_empty() {
                let count = (line.len() + 1) / 4;
                stacks.resize(count, Vec::new());
            } else {
                for (index, stack) in stacks.iter_mut().enumerate() {
                    let item = line.as_bytes()[1 + index * 4] as char;
                    if item != ' ' {
                        stack.push(item);
                    }
                }
            }
        }

        Self(stacks)
    }

    pub fn mv(&mut self, from: usize, to: usize, count: usize) {
        for _ in 0..count {
            let item = self.0[from].pop().unwrap();
            self.0[to].push(item);
        }
    }

    pub fn mv_multi(&mut self, from: usize, to: usize, count: usize) {
        let from = &mut self.0[from];
        let mut items = from[from.len() - count..].to_owned();
        from.truncate(from.len() - count);
        self.0[to].append(&mut items);
    }

    pub fn tops(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().map(|stack| *stack.last().unwrap())
    }

    pub fn _top(&self, stack: usize) -> Option<char> {
        self.0[stack].last().copied()
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, stack) in self.0.iter().enumerate() {
            writeln!(f, "{}: {:?}", index + 1, stack)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::get_input_vec;

    const PATH: &str = "../input/sample/day5.txt";

    #[test]
    fn test_stacks() {
        let input = get_input_vec(PATH);
        let lines = input.split(|l| l.is_empty()).next().unwrap();
        let mut s = Stacks::new(lines);
        println!("{}", s);
        assert_eq!('N', s._top(0).unwrap());
        assert_eq!('D', s._top(1).unwrap());
        assert_eq!('P', s._top(2).unwrap());

        s.mv(1, 0, 1);
        println!("{}", s);
        assert_eq!('D', s._top(0).unwrap());
        assert_eq!('C', s._top(1).unwrap());
        assert_eq!('P', s._top(2).unwrap());

        s.mv(0, 2, 3);
        println!("{}", s);
        assert!(s._top(0).is_none());
        assert_eq!('C', s._top(1).unwrap());
        assert_eq!('Z', s._top(2).unwrap());
    }

    #[test]
    fn test_mv_multi() {
        let input = get_input_vec(PATH);
        let lines = input.split(|l| l.is_empty()).next().unwrap();
        let mut s = Stacks::new(lines);

        s.mv_multi(1, 0, 1);
        println!("{}", s);
        assert_eq!('D', s._top(0).unwrap());
        assert_eq!('C', s._top(1).unwrap());
        assert_eq!('P', s._top(2).unwrap());

        s.mv_multi(0, 2, 3);
        println!("{}", s);
        assert!(s._top(0).is_none());
        assert_eq!('C', s._top(1).unwrap());
        assert_eq!('D', s._top(2).unwrap());
    }
}
