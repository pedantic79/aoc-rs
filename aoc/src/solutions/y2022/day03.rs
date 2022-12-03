use std::str::FromStr;

use aoc_harness::*;
use utils::numset::NumSet;

aoc_main!(2022 day 3, part1 [p1], part2 [p2], example both EG => (157,70));

const EG: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

fn score(c: char) -> usize {
    (if c.is_ascii_lowercase() {
        1 + (c as u8) - b'a'
    } else {
        27 + (c as u8) - b'A'
    }) as usize
}

struct ScoreSet(NumSet<u64>);

impl FromStr for ScoreSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ns: NumSet<u64> = s
            .chars()
            .map(score)
            .map(|x| u8::try_from(x).unwrap())
            .collect();
        Ok(ScoreSet(ns))
    }
}
impl ScoreSet {
    fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }
    fn single_score(&self) -> usize {
        self.0.iter().next().unwrap() as usize
    }
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let len = l.len();
            let (a, b) = l.split_at(len / 2);
            let a_set = ScoreSet::from_str(a).unwrap();
            let b_set = ScoreSet::from_str(b).unwrap();
            a_set.intersection(&b_set).single_score()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|l| ScoreSet::from_str(l).unwrap())
        .chunks(3)
        .into_iter()
        .map(|ch| ch.reduce(|a, b| a.intersection(&b)).unwrap().single_score())
        .sum()
}
