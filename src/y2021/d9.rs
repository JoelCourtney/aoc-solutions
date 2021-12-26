use anyhow::*;
use crate::Puzzle;
use std::collections::BinaryHeap;

const HEIGHT: usize = 100;
const WIDTH: usize = 100;
// const HEIGHT: usize = 5;
// const WIDTH: usize = 10;

type Grid<T> = [[T; WIDTH]; HEIGHT];

#[derive(Default)]
pub struct Day9;

impl Puzzle for Day9 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 9;
    type Input = Grid<u8>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Self::Input> {
        let mut grid = [[0_u8; WIDTH]; HEIGHT];
        let lines: Vec<_> = text.trim().lines().collect();
        assert_eq!(lines.len(), HEIGHT);
        for i in 0..HEIGHT {
            let digits: Vec<char> = lines[i].chars().collect();
            assert_eq!(digits.len(), WIDTH);
            for j in 0..WIDTH {
                grid[i][j] = digits[j].to_digit(10).ok_or(anyhow!("not a digit"))? as u8;
            }
        }
        Ok(grid)
    }

    fn part1(self, grid: Self::Input) -> Result<Self::Output1> {
        let lows = low_points(&grid);
        let mut risk: u32 = 0;
        for (i,j) in lows {
            risk += 1 + grid[i][j] as u32;
        }
        Ok(risk)
    }

    fn part2(self, grid: Self::Input) -> Result<Self::Output2> {
        let mut basins: Grid<Option<usize>> = [[None; WIDTH]; HEIGHT];
        let mut basin_counter = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if grid[i][j] != 9 {
                    let left_basin = if j > 0 {
                        basins[i][j-1]
                    } else {
                        None
                    };
                    let upper_basin = if i > 0 {
                        basins[i-1][j]
                    } else {
                        None
                    };
                    match (left_basin, upper_basin) {
                        (None, None) => {
                            basins[i][j] = Some(basin_counter);
                            basin_counter += 1;
                        }
                        // (None, Some(_)) if grid[i][j] == grid[i][j-1] => {
                        //     *basins[i][j] = Some(basin_counter);
                        //     basin_counter += 1;
                        // }
                        // (Some(_), None) if grid[i][j] == grid[i-1][j] => {
                        //     *basins[i][j] = Some(basin_counter);
                        //     basin_counter += 1;
                        // }
                        (None, Some(id)) | (Some(id), None) => {
                            basins[i][j] = Some(id);
                        }
                        (Some(id1), Some(id2)) if id1 == id2 => {
                            basins[i][j] = Some(id1);
                        }
                        (Some(id_left), Some(id_up)) => {
                            merge_basins(&mut basins, id_left, id_up, i, j);
                            basins[i][j] = Some(id_up);
                        }
                    }
                }
            }
        }
        let mut basin_sizes = vec![0_u32; basin_counter];
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if let Some(id) = basins[i][j] {
                    basin_sizes[id] += 1;
                }
            }
        }
        let mut heap = BinaryHeap::new();
        heap.extend(basin_sizes);
        Ok(heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap())
    }
}

fn merge_basins(basins: &mut Grid<Option<usize>>, from: usize, to: usize, mut i: usize, mut j: usize) {
    let mut found_in_row = true;
    while found_in_row {
        found_in_row = false;
        loop {
            if basins[i][j] == Some(from) {
                basins[i][j] = Some(to);
                found_in_row = true;
            }
            if j != 0 {
                j -= 1;
            } else {
                break;
            }
        }
        j = WIDTH - 1;
        if i != 0 {
            i -= 1;
        } else {
            break;
        }
    }
}

fn low_points(grid: &Grid<u8>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let mut neighbors = Vec::with_capacity(4);
            if i > 0 {
                neighbors.push((i-1, j));
            }
            if i < HEIGHT-1 {
                neighbors.push((i+1, j));
            }
            if j > 0 {
                neighbors.push((i, j-1));
            }
            if j < WIDTH-1 {
                neighbors.push((i, j+1));
            }
            if neighbors.into_iter().all(|(a,b)| grid[a][b] > grid[i][j]) {
                low_points.push((i,j));
            }
        }
    }
    low_points
}
