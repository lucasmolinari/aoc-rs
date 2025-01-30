use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
};

use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
};

#[derive(Debug, Default)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: i64,
    pub height: i64,
}
impl<T> Grid<T> {
    pub fn new(width: i64, height: i64) -> Self {
        Self {
            data: Vec::with_capacity((width * height) as usize),
            height,
            width,
        }
    }

    pub fn get(&self, point: Coord) -> &T {
        &self.data[(point.y * self.width + point.x) as usize]
    }

    pub fn set(&mut self, point: Coord, item: T) {
        self.data[(point.y * self.width + point.x) as usize] = item;
    }

    pub fn contains(&self, point: Coord) -> bool {
        point.x < self.width && point.y < self.height && point.x >= 0 && point.y >= 0
    }

    pub fn find(&self, item: T) -> Option<Coord>
    where
        T: PartialEq,
    {
        self.data.iter().position(|it| it == &item).map(|i| Coord {
            x: i as i64 % self.width,
            y: i as i64 / self.width,
        })
    }

    pub fn print(&self)
    where
        T: Display,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let item = self.get(Coord { x, y });
                print!("{}", item);
            }
            println!();
        }
    }

    pub fn iter(&self) -> GridIterator<'_, T> {
        GridIterator {
            grid: self,
            pos: Coord::default(),
        }
    }

    pub fn iter_from(&self, start: Coord) -> GridIterator<'_, T> {
        let x = start.x.clamp(0, self.width - 1);
        let y = start.y.clamp(0, self.width - 1);
        GridIterator {
            grid: self,
            pos: Coord::new(x, y),
        }
    }

    pub fn set_all<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Coord) -> T,
        T: Send,
    {
        self.data = (0..self.width * self.height)
            .into_par_iter()
            .map(|i| {
                setter(Coord {
                    x: i % self.width,
                    y: i / self.width,
                })
            })
            .collect();
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord) -> &Self::Output {
        &self.data[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[(self.width * index.y + index.x) as usize]
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    pos: Coord,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y >= self.grid.height {
            return None;
        }

        let coord = Coord::new(self.pos.x, self.pos.y);
        let item = self.grid.get(coord);
        self.pos.x += 1;
        if self.pos.x >= self.grid.width {
            self.pos.x = 0;
            self.pos.y += 1;
        }
        Some((coord, item))
    }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Hash, Clone, Copy, Eq, PartialEq)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn clockwise(self) -> Self {
        Coord::new(-self.y, self.x)
    }

    pub fn counter_clockwise(self) -> Self {
        Coord::new(self.y, -self.x)
    }
}
impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coord::new(0, -1),
            Direction::Down => Coord::new(0, 1),
            Direction::Left => Coord::new(-1, 0),
            Direction::Right => Coord::new(1, 0),
        }
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
