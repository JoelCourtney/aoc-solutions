use crate::Puzzle;
use std::str::FromStr;
use anyhow::Result;

#[derive(Default)]
pub struct Day2 {
    length: i32,
    depth: i32,
    aim: i32
}

impl Day2 {
    fn do_command1(&mut self, command: Command) {
        match command {
            Command::Horizontal(h) => self.length += h,
            Command::Vertical(v) => self.depth += v
        }
    }

    fn do_command2(&mut self, command: Command) {
        match command {
            Command::Vertical(v) => self.aim += v,
            Command::Horizontal(h) => {
                self.length += h;
                self.depth += self.aim * h;
            }
        }
    }
}

pub enum Command {
    Horizontal(i32),
    Vertical(i32)
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(if s.starts_with("forward") {
            Command::Horizontal(s["forward ".len()..].parse()?)
        } else if s.starts_with("up") {
            Command::Vertical(-s["up ".len()..].parse()?)
        } else if s.starts_with("down") {
            Command::Vertical(s["down ".len()..].parse()?)
        } else {
            anyhow::bail!("command not recognized: {}", s)
        })
    }
}

impl Puzzle for Day2 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 2;
    type Input = Vec<Command>;
    type Output1 = i32;
    type Output2 = i32;

    fn parse(text: String) -> Result<Self::Input> {
        Ok(text.lines().map(|s| s.parse().unwrap()).collect())
    }

    fn part1(mut self, input: Self::Input) -> Result<Self::Output1> {
        for command in input {
            self.do_command1(command);
        }
        Ok(self.depth * self.length)
    }

    fn part2(mut self, input: Self::Input) -> Result<Self::Output2> {
        for command in input {
            self.do_command2(command);
        }
        Ok(self.depth * self.length)
    }
}