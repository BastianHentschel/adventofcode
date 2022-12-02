use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Seek};
use std::mem::swap;

fn main() {
    let mut file = File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(file.try_clone().unwrap());
    let result1 = part1(reader.lines());
    file.rewind().unwrap();
    let reader = BufReader::new(file);
    let result2 = part2(reader.lines());
    println!("Day 1:");
    println!("  Result 1: {}", result1);
    println!("  Result 2: {}", result2);
}

fn part1(lines: impl Iterator<Item=io::Result<String>>) -> impl Display {
    let mut cur_count = 0;
    let mut max_count = 0;
    for line in lines {
        if let Ok(data) = line {
            match data.as_str() {
                "" =>
                    (max_count, cur_count) = (if max_count < cur_count { cur_count } else { max_count }, 0),

                _ => cur_count += data.parse::<i32>().unwrap(),
            }
        }
    }
    max_count
}

fn part2(lines: impl Iterator<Item=io::Result<String>>) -> impl Display {
    let mut max_counts = [0, 0, 0];
    let mut cur_count = 0;
    for line in lines {
        if let Ok(data) = line {
            match data.as_str() {
                "" => {
                    for count in max_counts.iter_mut() {
                        if *count < cur_count {
                            swap(count, &mut cur_count);
                        }
                    };
                    cur_count = 0;
                }

                _ => cur_count += data.parse::<i32>().unwrap(),
            }
        }
    }
    max_counts.iter().sum::<i32>()
}