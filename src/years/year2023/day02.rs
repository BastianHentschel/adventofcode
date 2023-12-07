use crate::test_day;
use regex::Regex;
use std::cmp::max;
use std::io::BufRead;

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    let low_count_re = Regex::new(r"^(((1[012]|\d) red|(1[0123]|\d) green|(1[01234]|\d) blue)[,;] )*((1[012]|\d) red|(1[0123]|\d) green|(1[01234]|\d) blue)$").unwrap();
    input
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .filter_map(|(game, line)| {
            low_count_re
                .is_match(&line.split_once(':').unwrap().1[1..])
                .then_some({ game + 1 })
        })
        .sum::<usize>()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    let count_re = Regex::new(r"(\d+) red|(\d+) green|(\d+) blue").unwrap();
    input
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(game, line)| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            count_re
                .captures_iter(&line.split_once(':').unwrap().1[1..])
                .for_each(|c| {
                    if let Some(num) = c.get(1) {
                        red = max(red, num.as_str().parse().unwrap());
                    } else if let Some(num) = c.get(2) {
                        green = max(green, num.as_str().parse().unwrap());
                    } else if let Some(num) = c.get(3) {
                        blue = max(blue, num.as_str().parse().unwrap());
                    }
                });
            red * green * blue
        })
        .sum::<usize>()
}

test_day!(year2023, day02, "8", "2286");
