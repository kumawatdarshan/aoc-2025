use crate::day4::parser::PaperRoll;
use rayon::prelude::*;

pub fn solution(input: &str) -> u64 {
    let positions = PaperRoll::to_hashset(input);

    positions
        .par_iter()
        .filter(|&roll| roll.count_neighbours(&positions) < 4)
        .count() as u64
}

#[test]
fn it_works() {
    let input = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let result = solution(input);
    assert_eq!(result, 13);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 1518);
}
