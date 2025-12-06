#![allow(unused)]
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct PaperRoll {
    pub x: i32,
    pub y: i32,
}

impl PaperRoll {
    pub const NEIGHBOURS: [PaperRoll; 8] = [
        PaperRoll::new(-1, -1), // top-left
        PaperRoll::new(0, -1),  // top
        PaperRoll::new(1, -1),  // top-right
        PaperRoll::new(-1, 0),  // left
        PaperRoll::new(1, 0),   // right
        PaperRoll::new(-1, 1),  // bottom-left
        PaperRoll::new(0, 1),   // bottom
        PaperRoll::new(1, 1),   // bottom-right
    ];

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn to_hashset(input: &str) -> FxHashSet<Self> {
        input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.bytes().enumerate().filter_map(move |(y, char)| {
                    (char == b'@').then_some(PaperRoll::new(x as i32, y as i32))
                })
            })
            .collect::<FxHashSet<Self>>()
    }

    #[inline]
    pub fn count_neighbours(&self, positions: &FxHashSet<Self>) -> usize {
        Self::NEIGHBOURS
            .iter()
            .map(move |&offset| *self + offset)
            .filter(|neighbour| positions.contains(neighbour)) // check if set of all correct values contains the iterated neighbour or not
            // .take(4)
            // if it does exist, accumulate
            .count()
    }
}

impl std::ops::Add for PaperRoll {
    type Output = PaperRoll;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
