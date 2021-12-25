use crate::Puzzle;
use anyhow::*;
use std::cmp::min;

#[derive(Default)]
pub struct Day7;

impl Puzzle for Day7 {
    const YEAR: u32 = 2021;
    const DAY: u32 = 7;
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(text: String) -> Result<Self::Input> {
        Ok(text.trim_end().split(',').map(|s| s.parse().unwrap()).collect())
    }

    fn part1(self, mut input: Self::Input) -> Result<Self::Output1> {
        input.sort_unstable();
        let mut cost = u32::MAX;
        for m in input[input.len()/2]..input[input.len()/2+1]+1 {
            cost = min(cost, fuel(&input, m, |n| n));
        }
        Ok(cost)
    }

    fn part2(self, mut input: Self::Input) -> Result<Self::Output2> {
        input.sort_unstable();
        let mut cost = u32::MAX;
        for m in 0..input[input.len()-1]+1 {
            let new_cost = fuel(&input, m, |n| n*(n+1)/2);
            if new_cost < cost {
                cost = new_cost;
            } else {
                break;
            }
        }
        Ok(cost)
    }
}

fn fuel(input: &Vec<u32>, median: u32, distance_func: impl Fn(u32) -> u32) -> u32 {
    let mut fuel = 0;
    for creb in input {
        if median > *creb {
            fuel += distance_func(median - creb);
        } else {
            fuel += distance_func(creb - median);
        }
    }
    fuel
}