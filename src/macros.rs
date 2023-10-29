

#[macro_export]
macro_rules! test_day {
    ($year:ident, $day:ident, $sol1:expr, $sol2:expr) => {
        #[cfg(test)]
        mod tests{
        mod $year {
            mod $day {
                use super::super::super::*;
                use std::io::BufReader;
                use std::fs::File;
                use std::path::PathBuf;
                #[test]
                fn test_part1() {
                    let path = PathBuf::new().join("data").join(format!("{}", stringify!($year))).join(format!("test_{}.txt", stringify!($day)));
                    let file = File::open(path).expect(&*format!("Missing input data for {} {}.", stringify!($year), stringify!($day)));
                    assert_eq!(part1(BufReader::new(file)), $sol1);
                    let path = PathBuf::new().join("data").join(format!("{}", stringify!($year))).join(format!("{}.txt", stringify!($day)));
                    let file = File::open(path).expect(&*format!("Missing input data for {} {}.", stringify!($year), stringify!($day)));
                    println!("{}", part1(BufReader::new(file)));
                }
                #[test]
                fn test_part2() {
                    let path = PathBuf::new().join("data").join(format!("{}", stringify!($year))).join(format!("test_{}.txt", stringify!($day)));
                    let file = File::open(path).expect(&*format!("Missing input data for {} {}.", stringify!($year), stringify!($day)));
                    assert_eq!(part2(BufReader::new(file)), $sol2);
                    let path = PathBuf::new().join("data").join(format!("{}", stringify!($year))).join(format!("{}.txt", stringify!($day)));
                    let file = File::open(path).expect(&*format!("Missing input data for {} {}.", stringify!($year), stringify!($day)));
                    println!("{}", part2(BufReader::new(file)));
                }
            }
        }
    }
};
}

#[macro_export]
macro_rules! test_test_existence {
    ($year:ident, $day:ident) => {
        mod $day {
            use std::fs::File;
            use std::io::{BufRead, BufReader};
            use std::path::PathBuf;
            use regex::Regex;
            #[test]
            fn test_test() {
                let path = PathBuf::new().join("src").join("years").join(stringify!($year)).join(format!("{}.rs", stringify!($day)));
                let file = File::open(path).expect(&*format!("Missing input data for {} {}.", stringify!($year), stringify!($day)));
                let file = BufReader::new(file);
                let regex = Regex::new(&*format!("test_day!\\(\\s*{}\\s*,\\s*{}\\s*,\\s*-?\\s*\\d+\\s*,\\s*-?\\s*\\d+\\s*\\)", stringify!($year), stringify!($day))).unwrap();
                let found_test = file.lines().find(|line| regex.is_match(line.as_ref().unwrap())).is_some();
                assert!(found_test);
            }
        }
    };
}