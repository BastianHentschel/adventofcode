use std::cmp::max;
use std::io::{BufRead};
use crate::test_day;

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> i32 {
    let mut highest = 0;
    let mut sum = 0;
    for line in input.lines() {
        let line = line.unwrap();
        if let Ok(num) = line.parse::<i32>() {
            sum += num;
        } else {
            highest = max(highest, sum);
            sum = 0;
        }
    }
    highest
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> i32 {
    let mut sums = vec![];

    let mut sum = 0;
    for line in input.lines() {
        let line = line.unwrap();
        if let Ok(num) = line.parse::<i32>() {
            sum += num;
        } else {
            sums.push(sum);
            sum = 0;
        }
    }
    if sum != 0 {
        sums.push(sum);
    }
    sums.sort_unstable();
    sums.iter().rev().take(3).sum()
}

test_day!(year2022, day01, "24000", "45000");
