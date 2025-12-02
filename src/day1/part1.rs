use std::str::FromStr;

use crate::day1::{INITIAL_VALUE, parser::Direction};

fn solution(input: &str) -> usize {
    input
        .split_whitespace()
        .map(Direction::from_str)
        .map(Result::unwrap)
        .scan(INITIAL_VALUE, |current, y| {
            *current = y.part1(*current);
            Some(*current)
        })
        .filter(|&x| x == 0)
        .count()
}

#[test]
fn it_works() {
    let input = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
    ";
    let result = solution(input);
    assert_eq!(result, 3);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 1071);
}
