use regex::Regex;

use crate::test_day;
use std::io::BufRead;

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    let mut input = input.lines().map(Result::unwrap);
    let times = input.next().unwrap();
    let distances = input.next().unwrap();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut times = num_regex.find_iter(&times);
    let mut distances = num_regex.find_iter(&distances);

    times
        .zip(distances)
        .map(|(time, distance)| {
            let time = time.as_str().parse::<usize>().unwrap();
            let distance = distance.as_str().parse::<usize>().unwrap();
            let mut count = 0;
            for used_time in 0..time {
                count += (used_time * (time - used_time) > distance) as usize;
            }
            count
        })
        .fold(1, usize::wrapping_mul)
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    let mut input = input.lines().map(Result::unwrap);
    let times = input.next().unwrap();
    let distances = input.next().unwrap();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut times = num_regex.find_iter(&times);
    let mut distances = num_regex.find_iter(&distances);

    let time = times.fold("".to_string(), |mut str, m| {str.push_str(m.as_str()); str});
    let distance = distances.fold("".to_string(), |mut str, m| {str.push_str(m.as_str()); str});
    let time = time.parse::<usize>().unwrap();
    let distance = distance.as_str().parse::<usize>().unwrap();
    let mut count = 0;
    for used_time in 0..time {
        count += (used_time * (time - used_time) > distance) as usize;
    }
    count
}

test_day!(year2023, day06, "288", "71503");
