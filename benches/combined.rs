const INPUT: &str = include_str!("../src/day4/input.txt");

#[divan::bench]
fn bench_day4_part1() -> u64 {
    aoc_2025::day4::part1::solution(divan::black_box(INPUT))
}

#[divan::bench]
fn bench_day4_part2() -> usize {
    aoc_2025::day4::part2::solution(divan::black_box(INPUT))
}

fn main() {
    divan::main();
}
