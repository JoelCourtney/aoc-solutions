use std::str::FromStr;
use anyhow::Result;
use shrinkwraprs::Shrinkwrap;
use crate::Puzzle;
use std::fmt::Debug;

const WIDTH: usize = 5;

#[derive(Default)]
pub struct Day4;

impl Puzzle for Day4 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 4;
    type Input = Game;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Self::Input> {
        text.parse()
    }

    fn part1(self, mut input: Self::Input) -> Result<Self::Output1> {
        loop {
            let n = input.numbers.pop().ok_or(anyhow::anyhow!("no one won"))?;
            for board in &mut input.boards {
                board.step(n);
                if board.won() {
                    dbg!(&board);
                    return Ok(n * board.sum_unmarked());
                }
            }
        }
    }

    fn part2(self, mut input: Self::Input) -> Result<Self::Output2> {
        let mut last = None;
        loop {
            let n = match input.numbers.pop() {
                Some(n) => n,
                None => break
            };
            input.boards.iter_mut().for_each(|board| board.step(n));
            input.boards = input.boards.into_iter().filter(|board| {
                if board.won() {
                    last = Some(n * board.sum_unmarked());
                    false
                } else {
                    true
                }
            }).collect();
        }
        last.ok_or(anyhow::anyhow!("no one won"))
    }
}

#[derive(Debug)]
pub struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<_> = s.lines().collect();
        let numbers = lines[0].split(',').rev().map(|s| s.parse().unwrap()).collect();
        let mut index = 2;
        let mut boards = Vec::new();
        while index < lines.len()-WIDTH+1 {
            boards.push(Board::from_strs(&lines[index..index+WIDTH])?);
            index += WIDTH + 1;
        }
        Ok(Game {
            numbers,
            boards
        })
    }
}
#[derive(Shrinkwrap, Debug)]
pub struct Board([[Spot; WIDTH]; WIDTH]);

impl Board {
    fn step(&mut self, n: u32) {
        for row in &mut self.0 {
            for spot in row {
                if *spot == n {
                    *spot = Spot::Marked;
                }
            }
        }
    }

    fn won(&self) -> bool {
        'outer: for i in 0..WIDTH {
            if self[i] == [Spot::Marked; WIDTH] {
                return true;
            }
            for j in 0..WIDTH {
                if let Spot::Unmarked(_) = self[j][i] {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut count = 0;
        for i in 0..WIDTH {
            for j in 0..WIDTH {
                if let Spot::Unmarked(n) = self[i][j] {
                    count += n;
                }
            }
        }
        count
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Spot {
    Unmarked(u32),
    Marked
}

impl PartialEq<u32> for Spot {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Spot::Unmarked(n) => *other == *n,
            Spot::Marked => false
        }
    }

    fn ne(&self, other: &u32) -> bool {
        match self {
            Spot::Unmarked(n) => *other != *n,
            Spot::Marked => true
        }
    }
}

impl Board {
    fn from_strs(lines: &[&str]) -> Result<Self> {
        assert_eq!(lines.len(), WIDTH);
        let mut result = [[Spot::Marked; WIDTH]; WIDTH];
        for i in 0..WIDTH {
            let numbers: Vec<_> = lines[i].split_ascii_whitespace().collect();
            assert_eq!(numbers.len(), WIDTH);
            for j in 0..WIDTH {
                result[i][j] = Spot::Unmarked(numbers[j].parse()?);
            }
        }
        Ok(Board(result))
    }
}