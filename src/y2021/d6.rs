use crate::Puzzle;
use anyhow::*;

const MAX: usize = 9;
type Int = u64;

#[derive(Default)]
pub struct Day6;

impl Puzzle for Day6 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 6;
    type Input = [Int; MAX];
    type Output1 = Int;
    type Output2 = Int;

    fn parse(text: String) -> Result<Self::Input> {
        let init: Vec<usize> = text.lines().nth(0).ok_or(anyhow!("no lines?"))?.split(',').map(|n| n.parse().unwrap()).collect();
        let mut result = [0; MAX];
        for i in init {
            result[i] += 1;
        }
        Ok(result)
    }

    fn part1(self, input: Self::Input) -> Result<Self::Output1> {
        let mut phish = input;
        for _ in 0..18 {
            phish = rabbit(phish);
        }
        let mut sum: Int = 0;
        for day in phish {
            sum += day;
        }
        Ok(sum)
    }

    fn part2(self, input: Self::Input) -> Result<Self::Output2> {
        let mut phish = input;
        for _ in 0..256 {
            phish = rabbit(phish);
        }
        let mut sum: Int = 0;
        for day in phish {
            sum += day;
        }
        Ok(sum)
    }
}

fn rabbit(input: [Int; MAX]) -> [Int; MAX] {
    let mut result = [0; MAX];
    for i in 1..input.len() {
        result[i-1] = input[i];
    }
    result[6] += input[0];
    result[8] += input[0];
    result
}
