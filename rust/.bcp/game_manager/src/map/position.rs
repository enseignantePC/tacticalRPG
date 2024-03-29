use std::ops::Add;

use super::*;
use gdnative::prelude::*;
// pub mod dijkstra;

pub use dijkstra_map::grids::Vector2D;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Pos2D(pub Vector2D<i32, i32>);
impl Pos2D {
    pub fn new(
        x: i32,
        y: i32,
    ) -> Self {
        let x = Vector2D::<i32, i32>::new(x, y);
        Pos2D(x)
    }
}

impl Deref for Pos2D {
    type Target = Vector2D<i32, i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToVariant for Pos2D {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        dict.insert("x", self.x);
        dict.insert("y", self.y);
        Variant::from_dictionary(&dict.into_shared())
    }
}

impl Add for &Pos2D {
    type Output = Pos2D;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Pos2D(self.0 + rhs.0)
    }
}
