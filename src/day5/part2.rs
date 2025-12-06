use std::ops::RangeInclusive;

use itertools::Itertools;
use rustc_hash::FxHashSet;

// this obviously works but is very slow solution
fn solution(s: &str) -> usize {
    let (id_range, _) = s.trim().split_once("\n\n").unwrap();

    let mut set: FxHashSet<u64> = FxHashSet::default();

    id_range
        .lines()
        .map(|x| {
            let (low, high) = x.split_once('-').unwrap();
            let low: u64 = low.parse().unwrap();
            let high: u64 = high.parse().unwrap();

            low..=high
        })
        .for_each(|x| {
            x.for_each(|value| {
                set.insert(value);
            });
        });

    set.len()
}

fn solution2(s: &str) -> u64 {
    #[inline(always)]
    fn range_len(r: RangeInclusive<u64>) -> u64 {
        r.end() - r.start() + 1
    }

    // coalesce fn dont actually fail, as you can see, the result has an error variant of OriginalRange,
    // meaning it just don't operate on the range if it can't merge it
    type OriginalRange = (RangeInclusive<u64>, RangeInclusive<u64>);
    fn merge_range(
        prev: RangeInclusive<u64>,
        curr: RangeInclusive<u64>,
    ) -> Result<RangeInclusive<u64>, OriginalRange> {
        let ranges_overlap = *curr.start() <= *prev.end() + 1;
        if ranges_overlap {
            let larger_end = (*prev.end()).max(*curr.end());
            let merged_range = *prev.start()..=larger_end;
            Ok(merged_range)
        } else {
            Err((prev, curr))
        }
    }

    let (id_range, _) = s.trim().split_once("\n\n").unwrap();

    let sorted_range_iter = id_range
        .lines()
        .map(|x| {
            let (low, high) = x.split_once('-').unwrap();
            let low: u64 = low.parse().unwrap();
            let high: u64 = high.parse().unwrap();

            low..=high
        })
        .sorted_by_key(|r| *r.start());

    let merged_range_iter = sorted_range_iter
        // coalesce does 2 things: gives adjacent item to work with
        // merges those 2 item based on a conditional
        // which being true, merges the item
        // if not, moves on to next
        .coalesce(merge_range)
        .map(range_len);

    merged_range_iter.sum::<u64>()
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
    let result = solution2(input);
    assert_eq!(result, 14);
}

#[test]
fn answer1() {
    let input = include_str!("./input.txt");

    let result = solution2(input);

    assert_eq!(result, 346240317247002);
}
