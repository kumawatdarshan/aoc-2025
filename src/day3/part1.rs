#![allow(unused)]

use itertools::Itertools;

fn concat_digits(a: &u32, b: &u32) -> u32 {
    let b_digits = b.ilog10() + 1;
    let res = a * 10u32.pow(b_digits) + b;
    dbg!(res);

    res
}
fn solution(input: &str) -> u32 {
    let bro: u32 = input
        .split_whitespace()
        .map(|x: &str| {
            let digits: Vec<u32> = x.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let n = digits.len();

            // max cannot be at last position, so we skip last digit
            let (max_idx, max_val) = digits[..n - 1]
                .iter()
                .enumerate()
                .rev() // fuck you max_by_key
                .max_by_key(|x| x.1)
                .unwrap();

            // skip the max_idx itself
            let next_max = digits[(max_idx + 1)..].iter().max().unwrap();

            concat_digits(max_val, next_max)
        })
        .sum();
    bro
}

#[test]
fn it_works() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let result = solution(input);
    assert_eq!(result, 357);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 16993);
}
