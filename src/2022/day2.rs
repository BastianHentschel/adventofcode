use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Seek};


fn main() {
    let mut file = File::open("data/day2.txt").unwrap();
    let reader = BufReader::new(file.try_clone().unwrap());
    let result1 = part1(reader.lines());
    file.rewind().unwrap();
    let reader = BufReader::new(file);
    let result2 = part2(reader.lines());
    println!("Day 2:");
    println!("  Result 1: {}", result1);
    println!("  Result 2: {}", result2);
}

fn part1(lines: impl Iterator<Item=io::Result<String>>) -> impl Display {
    let mut points = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut chars = line.chars();
            let enemy = chars.next().unwrap();
            let my = chars.last().unwrap();
            points += match my {
                'X' => 1 + match enemy {
                    'A' => 3,
                    'B' => 0,
                    'C' => 6,
                    _ => panic!(),
                },

                'Y' => 2 + match enemy {
                    'A' => 6,
                    'B' => 3,
                    'C' => 0,
                    _ => panic!(),
                },

                'Z' => 3 + match enemy {
                    'A' => 0,
                    'B' => 6,
                    'C' => 3,
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
    }
    points
}

fn part2(lines: impl Iterator<Item=io::Result<String>>) -> impl Display {
    let mut points = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut chars = line.chars();
            let enemy = chars.next().unwrap();
            let my = chars.last().unwrap();
            points += match my {
                'X' => 0 + match enemy {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => panic!(),
                },
                'Y' => 3 + match enemy {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => panic!(),
                },
                'Z' => 6 + match enemy {
                    'A' => 2,
                    'B' => 3,
                    'C' => 1,
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
    }
    points
}
