use regex::Regex;

use crate::test_day;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    input
        .lines()
        .map(|line| match count_num_intersections(&line.unwrap()) {
            0 => 0,
            i => 2usize.pow(i as u32 - 1),
        })
        .sum::<usize>()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    let mut count = 0;
    let mut additional = HashMap::new();
    for (i, line) in input.lines().map(Result::unwrap).enumerate() {
        let i = i + 1;
        count += 1 + additional.get(&i).unwrap_or(&0);
        let winning = count_num_intersections(&line);

        let multiplier = 1 + additional.get(&i).unwrap_or(&0);
        for row in i + 1..i + 1 + winning {
            *additional.entry(row).or_insert(0) += multiplier;
        }
    }
    count
}

fn count_num_intersections(line: &str) -> usize {
    let num_regex = Regex::new(r"\d+").unwrap();
    let (first, second) = (line.split_once(':').unwrap().1).split_once('|').unwrap();
    num_regex
        .find_iter(first)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<HashSet<_>>()
        .intersection(
            &num_regex
                .find_iter(second)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect::<HashSet<_>>(),
        )
        .count()
}
test_day!(year2023, day04, "13", "30");
