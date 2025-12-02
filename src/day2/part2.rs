use std::str::FromStr;

use crate::day2::parser::IdRange;
fn filter_invalid(id: &u64) -> bool {
    let id_str = id.to_string();
    let bytes = id_str.as_bytes();

    // for all possible chunk sizes
    for size in 1..=(bytes.len() / 2) {
        let mut chunks = bytes.chunks(size);

        if let Some(first) = chunks.next() // extract first chunk
        // check if they are equal
            && chunks.all(|c| c == first)
        {
            return true;
        }
    }

    false
}

// i am pretty sure i can improve perf in many areas but i like my functional solutions.
fn solution(input: &str) -> u64 {
    input
        .split(',')
        .map(IdRange::from_str)
        .map(Result::unwrap)
        .map(IntoIterator::into_iter)
        .map(|x| x.filter(filter_invalid).sum::<u64>())
        .sum()
}

#[test]
fn it_works() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let result = solution(input);
    assert_eq!(result, 4174379265);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 43287141963);
}
