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
    println!("Day 5:");
    println!("  Result 1: {}", result1);
    println!("  Result 2: {}", result2);
}

fn part1(reader: impl BufRead) -> impl Display {
    ""
}

fn part2(reader: impl BufRead) -> impl Display {
    ""
}

#[cfg(test)]
mod tests {
    use crate::aoc2022::day5;

    static DATA: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1() {
        let result = day5::part1(DATA.as_bytes());
        assert_eq!(format!("{}", result), "CMZ");
    }

    #[test]
    fn part2() {
        let result = day5::part2(DATA.as_bytes());
        assert_eq!(format!("{}", result), "4");
    }
}
