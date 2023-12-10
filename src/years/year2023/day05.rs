use regex::Regex;

use crate::test_day;
use std::{io::BufRead, ops::Range};

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut lines = input.lines().map(Result::unwrap);
    let num_slice = &lines.next().unwrap();
    let nums = num_regex.find_iter(&num_slice);
    let mut mappers = vec![];
    let mut lines = lines.skip_while(|l| !l.ends_with("map:")).skip(1);
    loop {
        let mut mapper = Mapper { ranges: vec![] };
        for line in (&mut lines).take_while(|l| !l.is_empty()) {
            let mut nums = num_regex.find_iter(&line);
            let target_start = nums.next().unwrap().as_str().parse::<usize>().unwrap();
            let source_start = nums.next().unwrap().as_str().parse::<usize>().unwrap();
            let range_length = nums.next().unwrap().as_str().parse::<usize>().unwrap();
            mapper.ranges.push((
                source_start..source_start + range_length,
                target_start as isize - source_start as isize,
            ));
        }
        mappers.push(mapper);
        if lines.next().is_none() {
            break;
        }
    }
    nums.map(|num| {
        let mut num = num.as_str().parse::<usize>().unwrap();
        for mapper in mappers.iter() {
            num = mapper.map(num).unwrap_or(num);
        }
        num
    })
    .min()
    .unwrap()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut lines = input.lines().map(Result::unwrap);
    let num_slice = &lines.next().unwrap();
    let nums = num_regex.find_iter(&num_slice);
    let mut mappers = vec![];
    let mut lines = lines.skip_while(|l| !l.ends_with("map:")).skip(1);
    loop {
        let mut mapper = Mapper { ranges: vec![] };
        for line in (&mut lines).take_while(|l| !l.is_empty()) {
            let mut nums = num_regex.find_iter(&line);
            let target_start = nums.next().unwrap().as_str().parse::<usize>().unwrap();
            let source_start = nums.next().unwrap().as_str().parse::<usize>().unwrap();
            let range_length = nums.next().unwrap().as_str().parse::<usize>().unwrap();
            mapper.ranges.push((
                source_start..source_start + range_length,
                target_start as isize - source_start as isize,
            ));
        }
        mappers.push(mapper);
        if lines.next().is_none() {
            break;
        }
    }
    nums.array_chunks::<2>()
        .map(|[range_start, length]| {
            let start = dbg!(range_start).as_str().parse::<usize>().unwrap();
            let length = length.as_str().parse::<usize>().unwrap();
            let range = start..start + length;
            range
                .map(|mut num| {
                    for mapper in &mappers {
                        num = mapper.map(num).unwrap_or(num);
                    }
                    num
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap(); 0
}

test_day!(year2023, day05, "35", "46");

struct Mapper {
    ranges: Vec<(Range<usize>, isize)>,
}

impl Mapper {
    fn map(&self, value: usize) -> Option<usize> {
        self.ranges.iter().find_map(|(range, offset)| {
            range
                .contains(&value)
                .then_some((value as isize + offset) as usize)
        })
    }
}
