use crate::Puzzle;
use anyhow::Result;
use std::str::FromStr;

#[derive(Default)]
pub struct Day1;

impl Puzzle for Day1 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 1;

    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Vec<u32>> {
        Ok(text.lines().map(|s| u32::from_str(s).unwrap()).collect())
    }

    fn part1(self, input: Vec<u32>) -> Result<u32> {
        Ok(input.iter().fold(
            (0, None),
                |acc, cur| {
                    if let Some(prev) = acc.1 {
                        if cur > prev {
                            (acc.0 + 1, Some(cur))
                        } else {
                            (acc.0, Some(cur))
                        }
                    } else {
                        (acc.0, Some(cur))
                    }
                }
        ).0)
    }

    fn part2(self, input: Vec<u32>) -> Result<u32> {
        let mut windows = Vec::new();
        for i in 2..input.len() {
            windows.push(input[i] + input[i-1] + input[i-2]);
        }
        self.part1(windows)
    }
}