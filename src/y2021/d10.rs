use crate::Puzzle;
use anyhow::*;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Default)]
pub struct Day10;

impl Puzzle for Day10 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 10;
    type Input = Vec<Vec<Delimiter>>;
    type Output1 = u32;
    type Output2 = u64;

    fn parse(text: String) -> Result<Self::Input> {
        Ok(text.trim().lines().map(|line| line.chars().map(|c| c.try_into().unwrap()).collect()).collect())
    }

    fn part1(self, input: Self::Input) -> Result<Self::Output1> {
        let mut score = 0;
        'next_line: for line in input {
            let mut stack = Vec::new();
            for delim in line {
                if delim.apply(&mut stack).is_err() {
                    score += delim.score()?;
                    continue 'next_line;
                }
            }
        }
        Ok(score)
    }

    fn part2(self, input: Self::Input) -> Result<Self::Output2> {
        use StackDelimiter::*;

        let mut scores = Vec::new();
        'next_line: for line in input {
            let mut stack = Vec::new();
            for delim in line {
                if let Err(_) = delim.apply(&mut stack) {
                    continue 'next_line;
                }
            }
            // bail!("meh");
            let mut score = 0;
            while let Some(delim) = stack.pop() {
                score *= 5;
                score += match delim {
                    Paren => 1,
                    Bracket => 2,
                    Brace => 3,
                    Hairpin => 4
                };
            }
            scores.push(score);
        }
        scores.sort_unstable();
        Ok(scores[scores.len() / 2])
    }
}

#[derive(Copy, Clone)]
pub enum Delimiter {
    OParen,
    CParen,
    OBracket,
    CBracket,
    OBrace,
    CBrace,
    OHairpin,
    CHairpin
}

impl Delimiter {
    fn apply(&self, stack: &mut Vec<StackDelimiter>) -> Result<()> {
        use Delimiter::*;

        match self {
            OParen | OBracket | OBrace | OHairpin => stack.push(StackDelimiter::from(*self)),
            _ => {
                if stack.pop().ok_or(anyhow!("can't pop"))? != StackDelimiter::from(*self) {
                    bail!("syntax error")
                }
            }

        }
        Ok(())
    }
    fn score(&self) -> Result<u32> {
        use Delimiter::*;

        Ok(match self {
            CParen => 3,
            CBracket => 57,
            CBrace => 1197,
            CHairpin => 25137,
            _ => bail!("not a close delimiter")
        })
    }
}

impl TryFrom<char> for Delimiter {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        use Delimiter::*;
        Ok(match c {
            '(' => OParen,
            ')' => CParen,
            '[' => OBracket,
            ']' => CBracket,
            '{' => OBrace,
            '}' => CBrace,
            '<' => OHairpin,
            '>' => CHairpin,
            o => bail!("character not valid: {}", o)
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StackDelimiter {
    Paren,
    Bracket,
    Brace,
    Hairpin
}

impl From<Delimiter> for StackDelimiter {
    fn from(de: Delimiter) -> Self {
        use Delimiter::*;
        use StackDelimiter::*;

        match de {
            OParen | CParen => Paren,
            OBracket | CBracket => Bracket,
            OBrace | CBrace => Brace,
            OHairpin | CHairpin => Hairpin,
        }
    }
}