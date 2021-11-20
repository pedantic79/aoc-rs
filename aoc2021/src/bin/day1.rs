use aoc2021::{aoc_gen, aoc_part};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let pd = aoc_gen!(parse_input);
    aoc_part!(pd, 1, part1);
    aoc_part!(pd, 2, part2);
}

fn parse_input(input: &str) -> HashSet<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &HashSet<u32>) -> u32 {
    let &n = input
        .iter()
        .find(|&&x| input.contains(&(2020 - x)))
        .unwrap();
    n * (2020 - n)
}
fn part2(input: &HashSet<u32>) -> u32 {
    input
        .iter()
        .combinations(2)
        .find_map(|x| {
            let sum = x[0] + x[1];
            if sum <= 2020 && input.contains(&(2020 - sum)) {
                Some(x[0] * x[1] * (2020 - sum))
            } else {
                None
            }
        })
        .unwrap()
}
