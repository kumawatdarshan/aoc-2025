use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

pub struct IdRange(u64, u64);

pub struct IdRangeIter {
    current: u64,
    end: u64,
}

impl IntoIterator for IdRange {
    type Item = u64;
    type IntoIter = RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0..=self.1
    }
}

impl FromStr for IdRange {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.trim().split_once('-').unwrap();

        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        Ok(Self(start, end))
    }
}
