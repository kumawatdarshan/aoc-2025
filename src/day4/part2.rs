use crate::day4::parser::PaperRoll;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

pub fn solution(input: &str) -> usize {
    let mut positions: FxHashSet<PaperRoll> = PaperRoll::to_hashset(input);

    let mut res = 0;

    loop {
        let to_remove: Vec<PaperRoll> = positions
            .par_iter() // Parallelism is still useful for the first massive pass
            .filter(|&roll| {
                let neighbors = PaperRoll::NEIGHBOURS
                    .iter()
                    .map(|&offset| *roll + offset)
                    .filter(|n| positions.contains(n))
                    .take(4)
                    .count();
                neighbors < 4
            })
            .copied()
            .collect();

        if to_remove.is_empty() {
            break;
        }

        to_remove.iter().for_each(|roll| {
            positions.remove(roll);
        });

        res += to_remove.len()
    }

    res
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
    assert_eq!(result, 43);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 8665);
}
