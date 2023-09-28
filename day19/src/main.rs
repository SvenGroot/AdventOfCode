// https://adventofcode.com/2022/day/19
#[macro_use]
extern crate text_io;

use std::{
    ops::{Add, Sub},
    path::Path,
    sync::mpsc,
    thread,
};

use aoc::{aoc_input, get_input};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let blueprints: Vec<_> = load_blueprints(path).collect();

    let mut sum = 0;
    for bp in &blueprints {
        let sim = Simulator {
            max_minute: 24,
            capacity: Resources {
                ore: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        let quality = bp.id as usize * sim.simulate(bp) as usize;
        println!("Blueprint {} has quality {}", bp.id, quality);
        sum += quality;
    }

    sum
}

// Pretty slow; took about 48 seconds.
fn part2(path: impl AsRef<Path>) -> usize {
    let blueprints: Vec<_> = load_blueprints(path).take(3).collect();

    let (send, recv) = mpsc::channel();
    for bp in blueprints {
        let send = send.clone();

        // Use a thread per blueprint to speed things up.
        thread::spawn(move || {
            let sim = Simulator {
                max_minute: 32,
                capacity: Resources {
                    ore: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            let geodes = sim.simulate(&bp) as usize;
            println!("Blueprint {} has output {}", bp.id, geodes);
            send.send(geodes).unwrap();
        });
    }

    drop(send);
    let mut product = 1;
    while let Ok(geodes) = recv.recv() {
        product *= geodes;
    }

    product
}

fn load_blueprints(
    path: impl AsRef<Path>,
) -> std::iter::Map<impl Iterator<Item = String>, impl FnMut(String) -> Blueprint> {
    get_input(path).map(|line| {
        let id: u32;
        let mut ore_robot = Robot::default();
        ore_robot.output.ore = 1;
        let mut clay_robot = Robot::default();
        clay_robot.output.clay = 1;
        let mut obs_robot = Robot::default();
        obs_robot.output.obsidian = 1;
        let mut geode_robot = Robot::default();
        geode_robot.output.geodes = 1;

        scan!(line.bytes() => "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            id, ore_robot.cost.ore, clay_robot.cost.ore, obs_robot.cost.ore, obs_robot.cost.clay, geode_robot.cost.ore, geode_robot.cost.obsidian);

        Blueprint {
            id,
            robots: [ore_robot, clay_robot, obs_robot, geode_robot]
        }
    })
}

#[derive(Default, Debug, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geodes: self.geodes - rhs.geodes,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Robot {
    output: Resources,
    cost: Resources,
}

impl Robot {
    fn can_build(&self, total: &Resources) -> bool {
        // Ignore geodes; not used to build bots.
        total.ore >= self.cost.ore
            && total.clay >= self.cost.clay
            && total.obsidian >= self.cost.obsidian
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    robots: [Robot; 4],
}

#[derive(Clone, Copy, Default)]
struct Simulator {
    prev_total: Resources,
    total: Resources,
    capacity: Resources,
    minute: u32,
    max_minute: u32,
}

impl Simulator {
    fn simulate(self, blueprint: &Blueprint) -> u32 {
        if self.minute == self.max_minute {
            return self.total.geodes;
        }

        // Don't build a new robot.
        let mut max_geodes = self.simulate_core(blueprint, Default::default());
        for robot in blueprint.robots {
            // Only consider a robot if you couldn't have built it on the previous minute.
            if robot.can_build(&self.total) && !robot.can_build(&self.prev_total) {
                let mut copy = self;
                copy.total = copy.total - robot.cost;
                max_geodes = max_geodes.max(copy.simulate_core(blueprint, robot.output));
            }
        }

        max_geodes
    }

    fn simulate_core(mut self, blueprint: &Blueprint, new_capacity: Resources) -> u32 {
        self.prev_total = self.total;
        self.total = self.total + self.capacity;
        self.capacity = self.capacity + new_capacity;
        self.minute += 1;
        self.simulate(blueprint)
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(33, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3472, part2(aoc_sample_input()));
    }
}
