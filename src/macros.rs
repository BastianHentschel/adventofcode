#[macro_export]
macro_rules! test_day {
    ($year:ident, $day:ident, $sol1:expr, $sol2:expr) => {
        #[cfg(test)]
        mod tests {
            mod $year {
                mod $day {
                    use super::super::super::*;
                    use std::fs::File;
                    use std::io::BufReader;
                    use std::path::PathBuf;
                    #[test]
                    fn test_part1() {
                        let path = PathBuf::new()
                            .join("data")
                            .join(format!("{}", stringify!($year)));
                        let file = File::open(path.join(format!("test_{}.txt", stringify!($day))))
                            .or_else(|_| {
                                File::open(
                                    path.join(format!("test_{}_part1.txt", stringify!($day))),
                                )
                            })
                            .expect(&*format!(
                                "Missing test case for {} {}",
                                stringify!($year),
                                stringify!($day)
                            ));
                        assert_eq!(part1(BufReader::new(file)).to_string(), $sol1.to_string());
                        let path = PathBuf::new()
                            .join("data")
                            .join(format!("{}", stringify!($year)))
                            .join(format!("{}.txt", stringify!($day)));
                        let file = File::open(path).expect(&*format!(
                            "Missing input data for {} {}.",
                            stringify!($year),
                            stringify!($day)
                        ));
                        println!("{}", part1(BufReader::new(file)).to_string());
                    }
                    #[test]
                    fn test_part2() {
                        let path = PathBuf::new()
                            .join("data")
                            .join(format!("{}", stringify!($year)));

                        let file = File::open(path.join(format!("test_{}.txt", stringify!($day))))
                            .or(File::open(
                                path.join(format!("test_{}_part2.txt", stringify!($day))),
                            ))
                            .expect(&*format!(
                                "Missing test case for {} {}",
                                stringify!($year),
                                stringify!($day)
                            ));
                        assert_eq!(part2(BufReader::new(file)).to_string(), $sol2.to_string());
                        let path = PathBuf::new()
                            .join("data")
                            .join(format!("{}", stringify!($year)))
                            .join(format!("{}.txt", stringify!($day)));
                        let file = File::open(path).expect(&*format!(
                            "Missing input data for {} {}.",
                            stringify!($year),
                            stringify!($day)
                        ));
                        println!("{}", part2(BufReader::new(file)).to_string());
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
            use regex::Regex;
            use std::fs::File;
            use std::io::{BufRead, BufReader};
            use std::path::PathBuf;
            #[test]
            fn test_test() {
                let path = PathBuf::new()
                    .join("src")
                    .join("years")
                    .join(stringify!($year))
                    .join(format!("{}.rs", stringify!($day)));
                let file = File::open(path).expect(&*format!(
                    "Missing input data for {} {}.",
                    stringify!($year),
                    stringify!($day)
                ));
                let file = BufReader::new(file);
                let regex = Regex::new(&*format!(
                    "test_day!\\(\\s*{}\\s*,\\s*{}\\s*,\\s*\".+\"\\s*\\,\\s*\".+\"\\s*\\)",
                    stringify!($year),
                    stringify!($day)
                ))
                .unwrap();
                let found_test = file
                    .lines()
                    .find(|line| regex.is_match(line.as_ref().unwrap()))
                    .is_some();
                assert!(found_test);
            }
        }
    };
}
