use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use std::mem::swap;
use std::path::Path;

pub fn runner<P: AsRef<Path>>(path: P) {
    let mut file = File::open(path).unwrap();
    let reader = BufReader::new(file.try_clone().unwrap());
    let result1 = part1(reader);
    file.rewind().unwrap();
    let reader = BufReader::new(file);
    let result2 = part2(reader);
    println!("Day 1:");
    println!("  Result 1: {}", result1);
    println!("  Result 2: {}", result2);
}

fn part1(reader: impl BufRead) -> impl Display {
    let mut cur_count = 0;
    let mut max_count = 0;
    let lines = reader.lines();
    for line in lines {
        if let Ok(data) = line {
            match data.as_str() {
                "" => {
                    max_count = if max_count < cur_count {
                        cur_count
                    } else {
                        max_count
                    };

                    cur_count = 0;
                }

                _ => cur_count += data.parse::<i32>().unwrap(),
            }
        }
    }
    max_count
}

fn part2(reader: impl BufRead) -> impl Display {
    let mut max_counts = [0, 0, 0];
    let mut cur_count = 0;
    let lines = reader.lines();
    for line in lines {
        if let Ok(data) = line {
            match data.as_str() {
                "" => {
                    for count in max_counts.iter_mut() {
                        if *count < cur_count {
                            swap(count, &mut cur_count);
                        }
                    }
                    cur_count = 0;
                }

                _ => cur_count += data.parse::<i32>().unwrap(),
            }
        }
    }
    for count in max_counts.iter_mut() {
        if *count < cur_count {
            swap(count, &mut cur_count);
        }
    }
    max_counts.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    static DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1() {
        let result = crate::part1(DATA.as_bytes());
        assert_eq!(format!("{}", result), "24000");
    }

    #[test]
    fn part2() {
        let result = crate::part2(DATA.as_bytes());
        assert_eq!(format!("{}", result), "45000");
    }
}
