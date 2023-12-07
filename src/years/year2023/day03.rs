use crate::test_day;
use regex::Regex;
use std::io::BufRead;
use std::iter::once;
use std::ops::{Not, Range};

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    let num_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^.\d]").unwrap();
    let empty_line =
        "............................................................................................................................................";
    once(empty_line.to_string())
        .chain(input.lines().map(Result::unwrap))
        .chain(once(empty_line.to_string()))
        .map_windows::<_, _, 3>(|window| {
            num_regex
                .find_iter(&window[1])
                .map(|m| {
                    ((symbol_regex
                        .is_match(&window[0][m.range().into_extended(1).bounded_by(&window[0])]))
                        || symbol_regex.is_match(
                            &window[2][m.range().into_extended(1).bounded_by(&window[0])],
                        )
                        || window[1]
                            .chars()
                            .nth(m.start().saturating_sub(1))
                            .unwrap_or('.')
                            .fulfills_any(&[char::is_ascii_digit, |c| c == &'.'])
                            .not()
                        || window[1]
                            .chars()
                            .nth(m.end())
                            .unwrap_or('.')
                            .fulfills_any(&[char::is_ascii_digit, |c| c == &'.'])
                            .not())
                    .then_some(m.as_str().parse::<usize>().unwrap())
                    .unwrap_or_default()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    let num_regex = Regex::new(r"\d+").unwrap();
    let num_regex_at_end = Regex::new(r"\d+$").unwrap();
    let num_regex_at_start = Regex::new(r"^\d+").unwrap();
    let symbol_regex = Regex::new(r"[^.\d]").unwrap();
    let empty_line =
        "............................................................................................................................................";
    once(empty_line.to_string())
        .chain(input.lines().map(Result::unwrap))
        .chain(once(empty_line.to_string()))
        .map_windows::<_, _, 3>(|window| {
            window[1]
                .match_indices('*')
                .map(|(gear_pos, _)| {
                    let mut iter = num_regex
                        .find_iter(&window[0])
                        .chain(num_regex.find_iter(&window[2]))
                        .filter(|m| dbg!(m.range()).overlaps(&(gear_pos..gear_pos + 1)))
                        .chain(num_regex_at_end.find_iter(window[1].get(..gear_pos).unwrap_or("")))
                        .chain(
                            num_regex_at_start
                                .find_iter(window[1].get(gear_pos + 1..).unwrap_or("")),
                        );

                    let x = iter.next().map_or(0, |first| {
                        dbg!(first.as_str());
                        iter.next().map_or(0, |second| {
                            dbg!(second.as_str());
                            iter.next().map_or_else(
                                || {
                                    first.as_str().parse::<usize>().unwrap()
                                        * second.as_str().parse::<usize>().unwrap()
                                },
                                |_| 0,
                            )
                        })
                    });
                    dbg!(x)
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

test_day!(year2023, day03, "4361", "467835");

trait RangeExt<T> {
    fn into_extended(self, val: T) -> Self;

    fn bounded_by(self, val: &str) -> Self;

    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeExt<usize> for Range<usize> {
    fn into_extended(self, val: usize) -> Self {
        Self {
            start: self.start.saturating_sub(val),
            end: self.end.saturating_add(val),
        }
    }

    fn bounded_by(self, val: &str) -> Self {
        Self {
            start: self.start.min(val.len() - 1),
            end: self.end.min(val.len() - 1),
        }
    }

    fn overlaps(&self, other: &Range<usize>) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

/// A trait for performing boolean operations on a type.
trait BoolOps {
    /// Checks if any of the provided functions return true when applied to the value.
    ///
    /// # Arguments
    ///
    /// * `funcs` - An array of functions that take a reference to the value and return a boolean.
    ///
    /// # Returns
    ///
    /// Returns true if any of the functions return true, otherwise false.
    fn fulfills_any(self, funcs: &[fn(&Self) -> bool]) -> bool;

    /// Checks if all of the provided functions return true when applied to the value.
    ///
    /// # Arguments
    ///
    /// * `funcs` - An array of functions that take a reference to the value and return a boolean.
    ///
    /// # Returns
    ///
    /// Returns true if all of the functions return true, otherwise false.
    fn fulfills_all(self, funcs: &[fn(&Self) -> bool]) -> bool;
}

impl<T> BoolOps for T {
    fn fulfills_any(self, funcs: &[fn(&Self) -> bool]) -> bool {
        funcs.iter().any(|func| func(&self))
    }

    fn fulfills_all(self, funcs: &[fn(&Self) -> bool]) -> bool {
        funcs.iter().all(|func| func(&self))
    }
}