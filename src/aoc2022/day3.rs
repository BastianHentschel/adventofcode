use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
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
    let mut sum: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let length = line.len(); // fine because only a-zA-Z as input
        let (comp1, comp2) = line.split_at(length / 2);
        let char = comp1.bytes()
            .find(|c| comp2.contains(*c as char))
            .expect("Missing char");
        sum += (char - if (char as char).is_ascii_lowercase() { 0x60 } else { 0x40 - 26 }) as u32;
    }
    sum
}

fn part2(reader: impl BufRead) -> impl Display {
    let mut sum: u32 = 0;
    let lines = reader.lines();
    for line_triple in lines.array_chunks::<3>() {
        let line_triple: Vec<&String> = line_triple.iter().map(|x| x.as_ref().unwrap()).collect();
        let char = line_triple[0].bytes()
            .find(|c| line_triple[1].contains(*c as char) && line_triple[2].contains(*c as char))
            .expect("Missing char");
        sum += (char - if (char as char).is_ascii_lowercase() { 0x60 } else { 0x40 - 26 }) as u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::aoc2022::day3;

    static DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        let result = day3::part1(DATA.as_bytes());
        assert_eq!(format!("{}", result), "157");
    }

    #[test]
    fn part2() {
        let result = day3::part2(DATA.as_bytes());
        assert_eq!(format!("{}", result), "70");
    }
}
