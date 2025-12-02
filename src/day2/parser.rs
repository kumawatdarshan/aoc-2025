use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

pub struct IdRange(u64, u64);

impl IntoIterator for IdRange {
    type Item = u64;

    type IntoIter = RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0..=self.1
    }
}

impl IdRange {
    fn new(start: u64, end: u64) -> Self {
        Self(start, end)
    }
}

// 2200670-2267527
impl FromStr for IdRange {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start, end)) = s.trim().split_once('-') else {
            panic!()
        };
        let start = start.parse().unwrap();
        let end = match end.parse() {
            Ok(x) => x,
            Err(e) => panic!("{end} caused {e}"),
        };

        Ok(Self::new(start, end))
    }
}
