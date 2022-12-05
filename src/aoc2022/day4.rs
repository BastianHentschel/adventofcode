use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use std::mem::swap;
use std::ops::Deref;
use std::path::Path;

pub fn runner<P: AsRef<Path>>(path: P) {
    let mut file = File::open(path).unwrap();
    let reader = BufReader::new(file.try_clone().unwrap());
    let result1 = part1(reader);
    file.rewind().unwrap();
    let reader = BufReader::new(file);
    let result2 = part2(reader);
    println!("Day 3:");
    println!("  Result 1: {}", result1);
    println!("  Result 2: {}", result2);
}

fn part1(reader: impl BufRead) -> impl Display {
    let mut count: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let ranges = line.split_once(',').unwrap();
        let range1 = ranges.0.split_once('-').unwrap();
        let range2 = ranges.1.split_once('-').unwrap();
        let range1 = range1.0.parse::<u32>().unwrap()..=range1.1.parse().unwrap();
        let range2 = range2.0.parse::<u32>().unwrap()..=range2.1.parse().unwrap();
        count += ((range1.start() <= range2.start() && range1.end() >= range2.end())
            || (range2.start() <= range1.start() && range2.end() >= range1.end())) as u32;
    };
    count
}

fn part2(reader: impl BufRead) -> impl Display {
    let mut count: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let ranges = line.split_once(',').unwrap();
        let range1_s = ranges.0.split_once('-').unwrap();
        let range2_s = ranges.1.split_once('-').unwrap();
        let mut range1 = range1_s.0.parse::<u32>().unwrap()..=range1_s.1.parse().unwrap();
        let mut range2 = range2_s.0.parse::<u32>().unwrap()..=range2_s.1.parse().unwrap();
        if range2.start() < range1.start() {
            swap(&mut range1, &mut range2);
        }

        count += (range2.start() <= range1.end()) as u32;
    };
    count
}

#[cfg(test)]
mod tests {
    use crate::aoc2022::day4;

    static DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        let result = day4::part1(DATA.as_bytes());
        assert_eq!(format!("{}", result), "2");
    }

    #[test]
    fn part2() {
        let result = day4::part2(DATA.as_bytes());
        assert_eq!(format!("{}", result), "4");
    }
}
