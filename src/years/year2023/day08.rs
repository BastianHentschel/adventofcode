use regex::Regex;

use crate::test_day;
use std::{collections::HashMap, io::BufRead};

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    let mut input = input.lines().map(Result::unwrap);
    let pos_regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let instructions = input.next().unwrap();
    let mut instructions = instructions.chars().cycle();
    input.next().unwrap();
    let positions = input
        .map(|line| {
            let caps = pos_regex.captures(&line).unwrap();
            (
                caps[1].to_string(),
                (caps[2].to_string(), caps[3].to_string()),
            )
        })
        .into_iter()
        .collect::<HashMap<_, _>>();
    let mut current = "AAA";
    for i in 1.. {
        match instructions.next().unwrap() {
            'L' => current = &positions[current].0,
            'R' => current = &positions[current].1,
            _ => panic!("shouldn't happen"),
        }
        if current == "ZZZ" {
            return i;
        }
    }
    unreachable!()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    let mut input = input.lines().map(Result::unwrap);
    let pos_regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let instructions = input.next().unwrap();
    let mut instructions = instructions.chars().cycle();
    input.next().unwrap();
    let positions = input
        .map(|line| {
            let caps = pos_regex.captures(&line).unwrap();
            (
                caps[1].to_string(),
                (caps[2].to_string(), caps[3].to_string()),
            )
        })
        .into_iter()
        .collect::<HashMap<_, _>>();
    let mut current = positions
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();
    for i in 1.. {
        let instruction = instructions.next().unwrap();
        for val in &mut current {
            match instruction {
                'L' => *val = &positions[*val].0,
                'R' => *val = &positions[*val].1,
                _ => panic!("shouldn't happen"),
            }
        }

        if current.iter().all(|pos| pos.ends_with('Z')) {
            return i;
        }
    }
    unreachable!()
}

test_day!(year2023, day08, "2", "6");
