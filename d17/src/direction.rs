use std::ops;
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl ops::Add<Direction> for Position {
    type Output = Self;

    fn add(mut self, rhs: Direction) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Up => self.y = self.y.wrapping_sub(1),
            Down => self.y += 1,
            Left => self.x = self.x.wrapping_sub(1),
            Right => self.x += 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    pub direction: Direction,
    pub cost: u32,
}

impl State {
    pub fn new(direction: Direction, cost: u32) -> Self {
        Self { direction, cost }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct States([State; 4]);

impl States {
    pub fn new(cost: u32) -> Self {
        Self([Up, Down, Left, Right].map(|direction| State { direction, cost }))
    }

    pub fn min(&self) -> u32 {
        self.0.iter().map(|state| state.cost).min().unwrap()
    }
}

impl ops::Index<Direction> for States {
    type Output = State;

    fn index(&self, index: Direction) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl ops::IndexMut<Direction> for States {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
