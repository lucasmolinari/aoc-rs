use std::fmt::{Display, Formatter};

use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
};

pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}
impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: Vec::with_capacity(width * height),
            height,
            width,
        }
    }

    pub fn get(&self, point: Point) -> &T {
        &self.data[point.y * self.width + point.x]
    }

    pub fn set(&mut self, point: Point, item: T) {
        self.data[point.y * self.width + point.x] = item;
    }

    pub fn print(&self)
    where
        T: Display,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let item = self.get(Point { x, y });
                print!("{}", item);
            }
            println!();
        }
    }

    pub fn set_all<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        self.data = (0..self.width * self.height)
            .into_par_iter()
            .map(|i| {
                setter(Point {
                    x: i % self.width,
                    y: i / self.width,
                })
            })
            .collect();
    }
}
