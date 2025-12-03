use std::str::FromStr;

use crate::day2::parser::IdRange;
fn filter_invalid(id_str: &str) -> bool {
    let bytes = id_str.as_bytes();
    let max_chunk_size = bytes.len() / 2;

    // for all possible chunk sizes
    for size in 1..=max_chunk_size {
        let mut chunks = bytes.chunks(size);
        let first = chunks.next().unwrap(); // extract first chunk
        // check if they are equal
        if chunks.all(|c| c == first) {
            return true;
        }
    }

    false
}

// i am pretty sure i can improve perf in many areas but i like my functional solutions.
// and i improved it.
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
    assert_eq!(result, 4174379265);
}

#[test]
pub fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 43287141963);
}
