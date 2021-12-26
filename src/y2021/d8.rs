use anyhow::*;
use crate::Puzzle;
use std::fmt::{Debug, Formatter};
use shrinkwraprs::Shrinkwrap;
use std::str::FromStr;
use std::convert::TryInto;

#[derive(Default)]
pub struct Day8;

impl Puzzle for Day8 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 8;
    type Input = Vec<SevenSeg>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Self::Input> {
        Ok(text.lines().map(|line| line.parse().unwrap()).collect())
    }

    fn part1(self, input: Self::Input) -> Result<Self::Output1> {
        let mut count = 0;
        for seg in input {
            for state in seg.values {
                count += state.info().is_known() as u32;
            }
        }
        Ok(count)
    }

    fn part2(self, input: Self::Input) -> Result<Self::Output2> {
        let mut sum = 0;
        for seg in input {
            sum += seg.resolve()?;
        }
        Ok(sum)
    }
}

#[derive(Debug)]
pub struct SevenSeg {
    digits: [State; 10],
    values: [State; 4]
}

impl SevenSeg {
    fn resolve(&self) -> Result<u32> {
        let one = self.digits.iter().find(|digit| digit.info() == StateInfo::Known(1)).ok_or(anyhow!("no one"))?;
        let seven = self.digits.iter().find(|digit| digit.info() == StateInfo::Known(7)).ok_or(anyhow!("no seven"))?;
        let four = self.digits.iter().find(|digit| digit.info() == StateInfo::Known(4)).ok_or(anyhow!("no four"))?;
        let eight = self.digits.iter().find(|digit| digit.info() == StateInfo::Known(8)).ok_or(anyhow!("no eight"))?;

        let mut two_three_five = self.digits.iter().filter(|digit| digit.info() == StateInfo::Unknown(5));
        let xor_235 = two_three_five.clone().fold(
            State::default(),
            |acc, next| acc.xor(next)
        );

        let three = two_three_five.clone().find(|digit| digit.xor(&xor_235).info() == StateInfo::Known(4)).ok_or(anyhow!("no three"))?;
        let two = two_three_five.clone().find(|digit| digit.xor(four).info() == StateInfo::Unknown(5)).ok_or(anyhow!("no two"))?;
        let five = two_three_five.find(
            |digit| digit.xor(four).info() == StateInfo::Known(7) && !digit.xor(three).is_blank()
        ).ok_or(anyhow!("no five"))?;

        let mut zero_six_nine = self.digits.iter().filter(|digit| digit.info() == StateInfo::Unknown(6));
        let six = zero_six_nine.clone().find(|digit| digit.xor(one).info() == StateInfo::Unknown(6)).ok_or(anyhow!("no six"))?;
        let zero = zero_six_nine.clone().find(|digit| digit.xor(five).info() == StateInfo::Known(7)).ok_or(anyhow!("no zero"))?;
        let nine = zero_six_nine.find(|digit| digit.xor(four).info() == StateInfo::Known(1)).ok_or(anyhow!("no nine"))?;

        let ordered = [zero, one, two, three, four, five, six, seven, eight, nine];
        let powers = [1000_u32, 100, 10, 1];
        let mut result = 0;
        'outer: for i in 0..4 {
            for j in 0..10_u32 {
                if self.values[i] == *ordered[j as usize] {
                    result += powers[i] * j;
                    continue 'outer;
                }
            }
            bail!("no match found: {:?}", self.values[i]);
        }
        Ok(result)
    }
}

impl FromStr for SevenSeg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let pipe = s.find('|').ok_or(anyhow!("no pipe"))?;
        let (digits, values) = s.split_at(pipe);

        fn fix<const N: usize>(iter: impl Iterator<Item=State>) -> Result<[State; N]> {
            let expanded: Vec<State> = iter.collect();
            expanded.try_into().map_err(|e: Vec<State>| anyhow!("found {} states instead of {}", e.len(), N))
        }

        Ok(SevenSeg {
            digits: fix(digits.split_ascii_whitespace().map(|s| s.parse().unwrap()))?,
            values: fix(values[2..].split_ascii_whitespace().map(|s| s.parse().unwrap()))?
        })
    }
}

#[derive(Shrinkwrap, Eq, PartialEq, Default, Copy, Clone)]
pub struct State([bool; 7]);

impl State {
    fn info(&self) -> StateInfo {
        let sum = self[0] as u32 +
            self[1] as u32 +
            self[2] as u32 +
            self[3] as u32 +
            self[4] as u32 +
            self[5] as u32 +
            self[6] as u32;
        match sum {
            2 => StateInfo::Known(1),
            4 => StateInfo::Known(4),
            3 => StateInfo::Known(7),
            7 => StateInfo::Known(8),
            s => StateInfo::Unknown(s)
        }
    }
    fn xor(&self, other: &State) -> State {
        State([
            self.0[0] ^ other.0[0],
            self.0[1] ^ other.0[1],
            self.0[2] ^ other.0[2],
            self.0[3] ^ other.0[3],
            self.0[4] ^ other.0[4],
            self.0[5] ^ other.0[5],
            self.0[6] ^ other.0[6],
        ])
    }
    fn is_blank(&self) -> bool {
        self.info() == StateInfo::Unknown(0)
    }
}

#[derive(Eq, PartialEq, Debug)]
enum StateInfo {
    Known(u32),
    Unknown(u32)
}

impl StateInfo {
    fn is_known(&self) -> bool {
        match self {
            StateInfo::Known(_) => true,
            _ => false
        }
    }
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = State([false; 7]);
        if s.contains('a') { state.0[0] = true; }
        if s.contains('b') { state.0[1] = true; }
        if s.contains('c') { state.0[2] = true; }
        if s.contains('d') { state.0[3] = true; }
        if s.contains('e') { state.0[4] = true; }
        if s.contains('f') { state.0[5] = true; }
        if s.contains('g') { state.0[6] = true; }

        Ok(state)
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = "".to_string();
        if self[0] { result.push('a'); }
        if self[1] { result.push('b'); }
        if self[2] { result.push('c'); }
        if self[3] { result.push('d'); }
        if self[4] { result.push('e'); }
        if self[5] { result.push('f'); }
        if self[6] { result.push('g'); }

        f.write_str(&result)
    }
}
