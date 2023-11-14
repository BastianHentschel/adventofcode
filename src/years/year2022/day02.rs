use std::io::{BufRead};
use crate::test_day;
use Hand::*;
use Outcome::*;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Hand {
    fn play(&self, other: &Self) -> Outcome {
        use Outcome::*;
        match self {
            Rock => match other {
                Rock => Draw,
                Paper => Loss,
                Scissors => Win,
            }
            Paper => match other {
                Rock => Win,
                Paper => Draw,
                Scissors => Loss,
            }

            Scissors => match other {
                Rock => Loss,
                Paper => Win,
                Scissors => Draw,
            }
        }
    }

    fn play_to(&self, other: &Outcome) -> Hand {
        match self {
            Rock => match other {
                Win => Paper,
                Draw => Rock,
                Loss => Scissors,
            }
            Paper => match other {
                Win => Scissors,
                Draw => Paper,
                Loss => Rock,
            }

            Scissors => match other {
                Win => Rock,
                Draw => Scissors,
                Loss => Paper,
            }
        }
    }
}

pub fn part1<R: BufRead>(input: R) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (opp, me) = line.split_once(' ').unwrap();
        let me = match me {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => unreachable!("fixed input")
        };

        let opp = match opp {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => unreachable!("fixed input")
        };
        score += match me.play(&opp) {
            Win => 6,
            Draw => 3,
            Loss => 0,
        };
        score += match me {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
    }

    score
}

pub fn part2<R: BufRead>(input: R) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (opp, target) = line.split_once(' ').unwrap();
        let target = match target {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!("fixed input")
        };

        let opp = match opp {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => unreachable!("fixed input")
        };
        let me = opp.play_to(&target);
        score += match target {
            Win => 6,
            Draw => 3,
            Loss => 0,
        };
        score += match me {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
    }
    score
}

test_day!(year2022, day02, "15", "12");
