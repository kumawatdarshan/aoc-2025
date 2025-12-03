use std::str::FromStr;

use crate::day2::parser::IdRange;
fn filter_invalid(id: &str) -> bool {
    // Now takes &str
    let half = id.len() / 2;
    let f_half = &id[..half];
    let s_half = &id[half..];
    f_half == s_half
}

pub fn solution(input: &str) -> u64 {
    let mut buffer = itoa::Buffer::new();

    input
        .split(',')
        .flat_map(|x| IdRange::from_str(x).unwrap())
        .fold(0, |acc, curr| {
            let id_str = buffer.format(curr);
            if filter_invalid(id_str) {
                acc + curr
            } else {
                acc
            }
        })
}

#[test]
fn it_works() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let result = solution(input);
    assert_eq!(result, 1227775554);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 34826702005);
}
