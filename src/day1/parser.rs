use std::str::FromStr;

const BOUNDARY: i32 = 100;

#[derive(Debug)]
pub enum Direction {
    Left(i32),
    Right(i32),
}

impl Direction {
    pub fn part1(&self, current: i32) -> i32 {
        match self {
            Direction::Left(value) => current - value,
            Direction::Right(value) => current + value,
        }
        .rem_euclid(BOUNDARY)
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_at(1);
        let direction = s.0;
        let value: i32 = s.1.parse().unwrap();

        match direction {
            "L" => Ok(Direction::Left(value)),
            "R" => Ok(Direction::Right(value)),
            _ => Err("Invalid direction".into()),
        }
    }
}
