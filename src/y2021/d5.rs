use anyhow::*;
use crate::Puzzle;
use std::fmt::Debug;
use std::str::FromStr;
use std::cmp::{min, max};

const WIDTH: usize = 1000;

#[derive(Default)]
pub struct Day5;

impl Puzzle for Day5 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 5;
    type Input = Vec<Line>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Self::Input> {
        Ok(text.lines().map(|line| line.parse().unwrap()).collect())
    }

    fn part1(self, input: Self::Input) -> Result<Self::Output1> {
        Ok(both_parts(input, false))
    }

    fn part2(self, input: Self::Input) -> Result<Self::Output2> {
        Ok(both_parts(input, true))
    }
}

fn both_parts(input: Vec<Line>, count_diagonal: bool) -> u32 {
    let mut grid = [[0_usize; WIDTH]; WIDTH];
    for line in input {
        if line.is_horizontal() {
            let x = line.x1;
            let min = min(line.y1, line.y2);
            let max = max(line.y1, line.y2);
            for y in min..max+1 {
                grid[x][y] += 1;
            }
        } else if line.is_vertical() {
            let y = line.y1;
            let min = min(line.x1, line.x2);
            let max = max(line.x1, line.x2);
            for x in min..max+1 {
                grid[x][y] += 1;
            }
        } else if count_diagonal {
            let (x_dir, length) = if line.x2 > line.x1 {
                (1, line.x2 - line.x1)
            } else {
                (-1, line.x1 - line.x2)
            };
            let y_dir = if line.y2 > line.y1 { 1 } else { -1 };
            for i in 0..length + 1 {
                grid[(line.x1 as i32 + i as i32 * x_dir) as usize][(line.y1 as i32 + i as i32 * y_dir) as usize] += 1;
            }
        }
    }
    let mut count = 0;
    for x in 0..WIDTH {
        for y in 0..WIDTH {
            if grid[x][y] > 1 {
                count += 1;
            }
        }
    }
    count
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.x1 == self.x2
    }
    fn is_vertical(&self) -> bool {
        self.y1 == self.y2
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let comma1 = s.find(',').ok_or(anyhow!("comma not found"))?;
        let space1 = s.find(" -> ").ok_or(anyhow!("arrow not found"))?;
        let comma2 = &s[space1+4..].find(',').ok_or(anyhow!("second comma not found"))?;

        let line = Line {
            x1: s[..comma1].parse()?,
            y1: s[comma1+1..space1].parse()?,
            x2: s[space1+4..(space1+4+comma2)].parse()?,
            y2: s[(space1+5+comma2)..].parse()?
        };
        Ok(line)
    }
}