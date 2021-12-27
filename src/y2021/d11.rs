use crate::Puzzle;
use anyhow::*;
use std::fmt::Debug;

const WIDTH: usize = 10;

type Grid<T> = [[T; WIDTH]; WIDTH];

#[derive(Default)]
pub struct Day11;

impl Puzzle for Day11 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 11;
    type Input = Grid<Octopus>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Self::Input> {
        let mut grid = [[Octopus::Flashed; WIDTH]; WIDTH];
        let lines: Vec<_> = text.trim().lines().collect();
        assert_eq!(lines.len(), WIDTH);
        for i in 0..WIDTH {
            assert_eq!(lines[i].len(), WIDTH);
            let chars: Vec<_> = lines[i].chars().collect();
            for j in 0..WIDTH {
                grid[i][j] = chars[j].into()
            }
        }
        Ok(grid)
    }

    fn part1(self, mut grid: Self::Input) -> Result<Self::Output1> {
        let mut flashes = 0;
        for _ in 0..100 {
            for i in 0..WIDTH {
                for j in 0..WIDTH {
                    if grid[i][j].step() {
                        flashes += 1 + expand_flash(&mut grid, i, j);
                    }
                }
            }
            for i in 0..WIDTH {
                for j in 0..WIDTH {
                    grid[i][j].reset();
                }
            }
        }
        Ok(flashes)
    }

    fn part2(self, mut grid: Self::Input) -> Result<Self::Output2> {
        let mut step = 1;
        loop {
            let mut flashes = 0;
            for i in 0..WIDTH {
                for j in 0..WIDTH {
                    if grid[i][j].step() {
                        flashes += 1 + expand_flash(&mut grid, i, j);
                    }
                }
            }
            if flashes == (WIDTH * WIDTH) as u32 {
                return Ok(step);
            }
            for i in 0..WIDTH {
                for j in 0..WIDTH {
                    grid[i][j].reset();
                }
            }
            step += 1;
        }
    }
}

fn expand_flash(grid: &mut Grid<Octopus>, i: usize, j: usize) -> u32 {
    let mut flashes = 0;
    if i > 0 {
        if j > 0 && grid[i-1][j-1].step() {
            flashes += 1 + expand_flash(grid, i-1, j-1);
        }
        if j < WIDTH-1 && grid[i-1][j+1].step() {
            flashes += 1 + expand_flash(grid, i-1, j+1);
        }
        if grid[i-1][j].step() {
            flashes += 1 + expand_flash(grid, i-1, j);
        }
    }
    if i < WIDTH-1 {
        if j > 0 && grid[i+1][j-1].step() {
            flashes += 1 + expand_flash(grid, i+1, j-1);
        }
        if j < WIDTH-1 && grid[i+1][j+1].step() {
            flashes += 1 + expand_flash(grid, i+1, j+1);
        }
        if grid[i+1][j].step() {
            flashes += 1 + expand_flash(grid, i+1, j);
        }
    }
    if j > 0 && grid[i][j-1].step() {
        flashes += 1 + expand_flash(grid, i, j-1);
    }
    if j < WIDTH-1 && grid[i][j+1].step() {
        flashes += 1 + expand_flash(grid, i, j+1);
    }
    flashes
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Octopus {
    Live(u8),
    Flashed
}

impl Octopus {
    fn step(&mut self) -> bool {
        if let Octopus::Live(level) = *self {
            if level == 9 {
                *self = Octopus::Flashed;
                true
            } else {
                *self = Octopus::Live(level+1);
                false
            }
        } else {
            false
        }
    }

    fn reset(&mut self) {
        if *self == Octopus::Flashed {
            *self = Octopus::Live(0);
        }
    }
}

impl From<char> for Octopus {
    fn from(c: char) -> Self {
        Octopus::Live(c.to_digit(10).unwrap() as u8)
    }
}