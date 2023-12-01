use std::io::BufRead;
use crate::test_day;

#[allow(unused)]
pub fn part1<R: BufRead>(input: R) -> impl ToString {
    let mut input = input.lines().map(Result::unwrap);
    let stack: Vec<_> = input.by_ref().take_while(|l| !l.is_empty()).collect();
    let stack_count = stack.last().unwrap().len() / 4 + 1;
    let mut stacks = vec![String::new(); stack_count];
    for line in stack.iter().rev().skip(1) {
        for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char != ' ' {
                stacks[i].push(char)
            }
        }
    }
    for line in input {
        let words: Vec<&str> = line.split(' ').collect();
        let amount: usize = words[1].parse().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        for _ in 0..amount {
            if let Some(char) = stacks[from].pop() {
                stacks[to].push(char);
            }
        }
    }

    stacks.into_iter().map(|mut s| s.pop().unwrap()).collect::<String>()
}

#[allow(unused)]
pub fn part2<R: BufRead>(input: R) -> impl ToString {
    let mut input = input.lines().map(Result::unwrap);
    let stack: Vec<_> = input.by_ref().take_while(|l| !l.is_empty()).collect();
    let stack_count = stack.last().unwrap().len() / 4 + 1;
    let mut stacks = vec![String::new(); stack_count];
    for line in stack.iter().rev().skip(1) {
        for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char != ' ' {
                stacks[i].push(char)
            }
        }
    }
    for line in input {
        let words: Vec<&str> = line.split(' ').collect();
        let amount: usize = words[1].parse().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        let split_pos = stacks[from].len() - amount;
        let take = stacks[from].split_off(split_pos);
        stacks[to].extend(take.chars());
    }

    stacks.into_iter().map(|mut s| s.pop().unwrap()).collect::<String>()
}

test_day!(year2022, day05, "CMZ", "MCD");