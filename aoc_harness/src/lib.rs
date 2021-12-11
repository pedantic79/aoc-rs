//Convenience re-exports

use std::{env, path::PathBuf, str::FromStr, time::Instant};

pub use aoc_harness_macros::*;
pub use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    /// Override the input with the contents of this file
    #[structopt(short, long)]
    pub input: Option<PathBuf>,
    #[structopt(short, long)]
    pub quiet: bool,
    ///panic if results don't match expected.
    #[structopt(short, long)]
    pub test_mode: bool,
    #[structopt(short, long, default_value = "1")]
    pub repeats: usize,
    /// Bypass lightweight benchmarking
    #[structopt(short, long)]
    pub bypass: bool,
}

impl Opts {
    pub fn for_test() -> Self {
        Self {
            input: None,
            quiet: true,
            test_mode: true,
            repeats: 1,
            bypass: true,
        }
    }
    pub fn log<F: FnOnce() -> String>(&self, f: F) {
        if !self.quiet {
            println!("{}", f());
        }
    }
    pub fn assert_eq<T: Eq + core::fmt::Debug>(&self, actual: T, expected: T) {
        if self.test_mode {
            assert_eq!(actual, expected);
        } else if actual != expected {
            self.log(|| {
                format!(
                    "!!! Answer does not match expected: {:?} != {:?}",
                    actual, expected
                )
            });
        }
    }
    pub fn get_input(&self, year: i32, day: u8) -> String {
        match &self.input {
            None => {
                //try in cache dir first.
                let p = PathBuf::from(format!(
                    "{}input/{}/day{:02}.txt",
                    std::env::var("CARGO_WORKSPACE_DIR").unwrap_or_else(|_| String::new()),
                    year,
                    day
                ));
                if !p.exists() {
                    std::fs::create_dir_all(p.parent().unwrap())
                        .expect("couldn't create year input dir");
                    let i = ureq::get(&format!(
                        "https://adventofcode.com/{}/day/{}/input",
                        year, day
                    ))
                    .set(
                        "cookie",
                        &format!(
                            "session={}",
                            env::var("AOC_SESSION").expect("AOC_SESSION env var not set")
                        ),
                    )
                    .call()
                    .unwrap()
                    .into_string()
                    .unwrap();
                    std::fs::write(p, &i).expect("failed to write cached input file");
                    i
                } else {
                    std::fs::read_to_string(p).expect("couldn't read cached input file")
                }
            }
            Some(f) => std::fs::read_to_string(f).expect("Couldn't read file"),
        }
    }
    pub fn time_fn<O, F>(&self, f: F) -> (std::time::Duration, O)
    where
        F: Fn() -> O,
    {
        let start = Instant::now();
        let ans = f();
        let end = Instant::now();
        let dur = end - start;
        let target_dur = std::time::Duration::new(0, 50_000_000);
        if !self.bypass && dur < target_dur {
            //took less than 50ms. How many could we do in 50ms?
            let c = (target_dur.as_secs_f64() / dur.as_secs_f64()) as usize;
            let start = Instant::now();
            for _ in 0..c {
                f();
            }
            let end = Instant::now();
            let overall = (end - start).as_secs_f64() / (c as f64);
            (std::time::Duration::from_secs_f64(overall), ans)
        } else {
            (dur, ans)
        }
    }
}

pub fn render_duration(d: std::time::Duration) -> String {
    let mut value = d.as_secs_f64();
    let units = ["s", "ms", "µs", "ns"];
    for u in units {
        if value > 1.0 {
            return format!("{:.3}{}", value, u);
        }
        value *= 1000.0;
    }
    "<1ns".to_string()
}

pub fn whole_input_is<O>(i: &str) -> O
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.parse().unwrap()
}
pub fn lines<O>(i: &str) -> Vec<O>
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.trim().lines().map(|x| x.parse().unwrap()).collect()
}
pub fn input<O: FromStr, const S: char>(i: &str) -> Vec<O>
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.trim().split(S).map(|x| x.parse().unwrap()).collect()
}
