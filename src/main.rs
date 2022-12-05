#![feature(iter_array_chunks)]

pub mod macros;

mod aoc2022;

fn main() {
    runners!(aoc2022, day1, day2, day3, day4);
}
