fn solution(s: &str) -> usize {
    let (id_range, id_to_check) = s.trim().split_once("\n\n").unwrap();

    let complete_range = id_range
        .lines()
        .map(|x| {
            let (low, high) = x.split_once('-').unwrap();
            let low: u64 = low.parse().unwrap();
            let high: u64 = high.parse().unwrap();

            low..=high
        })
        .collect::<Vec<_>>();

    id_to_check
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .filter(|x| complete_range.iter().any(|range| range.contains(x)))
        .count()
}

#[test]
fn it_works() {
    let input = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let result = solution(input);
    assert_eq!(result, 3);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution(input);

    assert_eq!(result, 896);
}
