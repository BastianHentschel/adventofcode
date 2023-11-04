use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::io::{BufRead};
use std::ops::RangeBounds;
use crate::test_day;

pub fn part1<R: BufRead>(input: R) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (r1, r2) = line.split_once(',').unwrap();

        let (s1, e1) = r1.split_once('-').unwrap();
        let (s2, e2) = r2.split_once('-').unwrap();
        let s1 = s1.parse::<i32>().unwrap();
        let e1 = e1.parse::<i32>().unwrap();
        let s2 = s2.parse::<i32>().unwrap();
        let e2 = e2.parse::<i32>().unwrap();

        count += (s1 <= s2 && e1 >= e2 || s1 >= s2 && e1 <= e2) as i32;

    }
    count
}

pub fn part2<R: BufRead>(input: R) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (r1, r2) = line.split_once(',').unwrap();

        let (s1, e1) = r1.split_once('-').unwrap();
        let (s2, e2) = r2.split_once('-').unwrap();
        let s1 = s1.parse::<i32>().unwrap();
        let e1 = e1.parse::<i32>().unwrap();
        let s2 = s2.parse::<i32>().unwrap();
        let e2 = e2.parse::<i32>().unwrap();

        count += (e1 >= s2 && s1 <= e2 || e2 >= s1 && s2 <= e1) as i32;

    }
    count
}

test_day!(year2022, day04, 2, 4);