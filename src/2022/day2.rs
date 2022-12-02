use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

fn main() {
    {// Part One
        let file = File::open("data/day2.txt").unwrap();
        let lines = BufReader::new(file).lines();
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
        println!("Result (1): {points}");
    }

    {// Part Two
        // Part One
        let file = File::open("data/day2.txt").unwrap();
        let lines = BufReader::new(file).lines();
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
        println!("Result (2): {points}");
    }
}