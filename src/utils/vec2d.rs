//! Mathematical Vector (2 dimensional)



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d<T> { pub x: T, pub y: T }

impl<T> Vec2d<T> {

    pub fn new(x: T, y: T) -> Self {
        Vec2d { x, y }
    }

    fn from_tuple(xy: (T, T)) -> Vec2d<T> {
        Vec2d { x: xy.0, y: xy.1 }
    }

}
