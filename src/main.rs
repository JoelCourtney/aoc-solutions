mod macros;
mod y2021;

use structopt::StructOpt;
use std::fmt::Debug;
use anyhow::Result;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Year to run
    #[structopt(short, long, default_value = "2021")]
    year: u32,

    /// Day to run
    #[structopt(short, long)]
    day: u32,

    /// Part to run
    #[structopt(short, long)]
    part: u32
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        dbg!(e);
    }
}

fn run(opt: Opt) -> Result<()> {
    matches! {
        opt;

        y2021::d1::Day1,
        y2021::d2::Day2,
        y2021::d3::Day3,
        y2021::d4::Day4,
        y2021::d5::Day5,
        y2021::d6::Day6,
        y2021::d7::Day7,
        y2021::d8::Day8,
        y2021::d9::Day9,
    }
    Ok(())
}

trait Puzzle: Default {
    const YEAR: u32;
    const DAY: u32;

    type Input;
    type Output1: Debug;
    type Output2: Debug;

    fn parse(text: String) -> Result<Self::Input>;

    fn part1(self, input: Self::Input) -> Result<Self::Output1>;
    fn part2(self, input: Self::Input) -> Result<Self::Output2>;
}