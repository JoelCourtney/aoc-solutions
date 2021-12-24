use crate::Puzzle;
use anyhow::Result;

const BITS: usize = 12;

#[derive(Default)]
pub struct Day3;

impl Puzzle for Day3 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 3;
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Self::Input> {
        Ok(text.lines().map(|s| u32::from_str_radix(s, 2).unwrap()).collect())
    }

    fn part1(self, input: Self::Input) -> Result<Self::Output1> {
        let (gamma, epsilon) = most_and_least_common(&input)?;
        Ok(gamma*epsilon)
    }

    fn part2(self, input: Self::Input) -> Result<Self::Output2> {
        let mut o2_candidates = input.clone();
        let mut co2_candidates = input.clone();
        for bit_index in (0..BITS).rev() {
            if o2_candidates.len() > 1 {
                let (gamma, _) = most_and_least_common(&o2_candidates)?;
                o2_candidates = o2_candidates.into_iter().filter(|n| {
                    ((*n ^ !gamma) & (1 << bit_index)) != 0
                }).collect();
                dbg!(&o2_candidates);
            }
            if co2_candidates.len() > 1 {
                let (_, epsilon) = most_and_least_common(&co2_candidates)?;
                dbg!(epsilon);
                co2_candidates = co2_candidates.into_iter().filter(|n| {
                    ((*n ^ !epsilon) & (1 << bit_index)) != 0
                }).collect();
                dbg!(&co2_candidates);
            }
        }
        assert_eq!(o2_candidates.len(), 1);
        assert_eq!(co2_candidates.len(), 1);
        Ok(o2_candidates.first().unwrap() * co2_candidates.first().unwrap())
    }
}

fn most_and_least_common(input: &Vec<u32>) -> Result<(u32, u32)> {
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    for bit_index in (0..BITS).rev() {
        let mut ones = 0;
        for num in input {
            ones += (*num & (1 << bit_index)) >> bit_index;
        }
        if (input.len() % 2 == 0 && ones == (input.len() / 2) as u32) || ones > (input.len() / 2) as u32 {
            gamma.push("1");
            epsilon.push("0");
        } else {
            gamma.push("0");
            epsilon.push("1");
        }
    }

    let gamma = u32::from_str_radix(&gamma.join(""),2)?;
    let epsilon = u32::from_str_radix(&epsilon.join(""),2)?;

    Ok((gamma, epsilon))
}