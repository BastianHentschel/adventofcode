use crate::test_day;
use std::io::BufRead;

use crate::fold_map::FoldMapExt;

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    input
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.next().unwrap();
            let last = iter.next_back().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const MAX_N: usize = 5; //NUMS.iter().map(|s| s.len()).max().unwrap();
    input
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut chars = line.chars();
            let mut iter = chars.fold_map(String::new(), |storage, item| {
                item.to_digit(10).map(|num| num as usize).or_else(|| {
                    storage.push(item);
                    NUMS.iter()
                        .position(|num| storage.ends_with(num))
                        .map(|i| i + 1)
                })
            });

            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);

            first * 10 + last
        })
        .sum::<usize>()
}

test_day!(year2023, day01, "142", "281");
