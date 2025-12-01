use crate::day1::parser::Move;

fn part1(input: &str) -> usize {
    input
        .split_whitespace()
        .map(Move::parse)
        .scan(50u8, |current, y| {
            *current = y.apply_op(*current);
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
    let result = part1(input);
    assert_eq!(result, 3);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = part1(input);

    assert_eq!(result, 1071);
}
