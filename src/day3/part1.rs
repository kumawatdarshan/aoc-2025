fn solution(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|x| {
            let digits: Vec<u64> = x.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
            super::pick_jolt(&digits, 2)
        })
        .sum()
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
