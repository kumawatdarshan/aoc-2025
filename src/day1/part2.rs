// #![allow(unused)]
use rstest::rstest;
use std::str::FromStr;

use crate::day1::{BOUNDARY, INITIAL_VALUE, parser::Direction};

pub fn count_passing_0(current: i32, prev: i32) -> (i32, i32) {
    let next = prev + current;
    let mut revolutions = (next / BOUNDARY).abs();

    // current != 0 is to check double counting.
    // next <= is to check if we went in reverse direction
    if current != 0 && next <= 0 {
        revolutions += 1;
    }

    (next.rem_euclid(BOUNDARY), revolutions)
}

fn solution(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(Direction::from_str)
        .map(Result::unwrap)
        .map(handle_direction)
        .scan(INITIAL_VALUE, |current, prev| {
            let (next, rotations) = count_passing_0(*current, prev);
            *current = next;
            Some(rotations)
        })
        .sum()
}

fn handle_direction(direction: Direction) -> i32 {
    match direction {
        Direction::Left(value) => -value,
        Direction::Right(value) => value,
    }
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
    assert_eq!(result, 6);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 6700);
}

#[rstest]
#[case(50,-68,1)]
#[case(82,-30,0)]
#[case(52, 48, 1)]
#[case(0,-5,0)]
#[case(95, 60, 1)]
#[case(55,-55,1)]
#[case(0,-1,0)]
#[case(99,-99,1)]
#[case(0, 14, 0)]
#[case(14,-82,1)]
fn spin_test(#[case] input: i32, #[case] current: i32, #[case] expected: i32) {
    let (_new, rotations) = count_passing_0(input, current);

    assert_eq!(expected, rotations);
}
