use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::io::{BufRead};
use crate::test_day;

pub fn part1<R: BufRead>(input: R) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (first, second) = line.split_at(line.len() / 2);
        let first_set = HashSet::<char, RandomState>::from_iter(first.chars());
        let second_set = HashSet::from_iter(second.chars());
        let char = first_set.intersection(&second_set).next().unwrap();
        sum += get_priority(*char) as i32
    }
    sum
}

pub fn part2<R: BufRead>(input: R) -> i32 {
    let mut sum = 0;
    let groups = input.lines().map(|x| x.unwrap()).array_chunks::<3>();
    for group in groups {
        let first_set = HashSet::<char, RandomState>::from_iter(group.get(0).unwrap().chars());
        let second_set = HashSet::from_iter(group.get(1).unwrap().chars());
        let third_set = HashSet::<char, RandomState>::from_iter(group.get(2).unwrap().chars());

        let itrsec1 = HashSet::from_iter(first_set.intersection(&second_set).map(|c| *c));
        let char = itrsec1.intersection(&third_set).next().unwrap();

        sum += get_priority(*char) as i32

    }
    sum
}

test_day!(year2022, day03, 157, 70);

fn get_priority(char: char) -> u8 {
    const Z: u8 = 'Z'.as_ascii().unwrap().as_u8();
    const A: u8 = 'A'.as_ascii().unwrap().as_u8();
    const a: u8 = 'a'.as_ascii().unwrap().as_u8();
    let ascii = char.as_ascii().unwrap().as_u8();
    ascii - if ascii <= Z {
        A - 26
    } else {
        a
    } + 1
}