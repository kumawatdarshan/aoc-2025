pub mod parser;
pub mod part1;
pub mod part2;

pub(super) fn pick_jolt(digits: &[u64], max_digit_len: usize) -> u64 {
    let len = digits.len();
    let mut start = 0;
    let mut result = 0u64;

    for i in 0..max_digit_len {
        // (len, max_digit_len, i) = (100, 12, 0)
        // end = 88, ie, skip 12
        // (len, max_digit_len, i) = (100, 12, 1)
        // end = 89, ie, skip 11
        // and so on
        let end = len - (max_digit_len - i);
        let window = &digits[start..=end];

        // find the new max_idx
        let (max_idx, max_val) = first_max_with_idx(window);
        // `idx` is number of digits passed from the left
        let idx = start + max_idx;

        result = concat(result, *max_val);

        // shrink next window
        start = idx + 1;
    }

    result
}

fn concat(out: u64, max_val: u64) -> u64 {
    out * 10 + max_val
}

fn first_max_with_idx(digits: &[u64]) -> (usize, &u64) {
    let (max_idx, max_val) = digits
        .iter()
        .enumerate() // extract index aswell
        .rev() // fuck you max_by_key
        .max_by_key(|x| x.1) // this gives the last occurence of the max if multiple max found
        .unwrap();
    (max_idx, max_val)
}
