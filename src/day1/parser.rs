use std::{
    ops::{Add, Deref, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub struct Move {
    direction: Direction,
    value: Wrapped100,
}

impl Move {
    pub fn parse(s: &str) -> Self {
        let (dir, value) = s.split_at(1);
        let direction = Direction::from_str(dir).unwrap();
        let value = Wrapped100::from_str(value).unwrap();

        Self { direction, value }
    }

    pub fn apply_op(&self, current: u8) -> u8 {
        let current = Wrapped100::new(current);
        match self.direction {
            Direction::Left => current - self.value,
            Direction::Right => current + self.value,
        }
        .0
    }

    pub(crate) fn rotation_count(&self, next: u8) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Invalid direction".into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Wrapped100(u8);

impl Wrapped100 {
    pub fn new<T: Into<usize>>(value: T) -> Self {
        Self((value.into() % 100) as u8)
    }
}

impl FromStr for Wrapped100 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: usize = s.parse().unwrap();

        Ok(Self::new(x))
    }
}

impl Deref for Wrapped100 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for Wrapped100 {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl Add for Wrapped100 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let z = *self + *rhs;
        Self::new(z)
    }
}

impl Sub for Wrapped100 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let z = (*self + 100) - *rhs;
        Self::new(z)
    }
}
