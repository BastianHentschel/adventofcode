use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

fn main() {
    {// Part One
        let file = File::open("data/day1.txt").unwrap();
        let lines = BufReader::new(file).lines();
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
        println!("Result (1): {}", max_count);
    }

    {// Part Two
        let file = File::open("data/day1.txt").unwrap();
        let lines = BufReader::new(file).lines();
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
        println!("Result (2): {:?}", max_counts.iter().sum::<i32>());
    }
}