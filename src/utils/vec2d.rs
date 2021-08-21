//! Mathematical Vector (2 dimensional)



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d<T> { pub x: T, pub y: T }

impl<T> Vec2d<T> {

    fn new(x: T, y: T) -> Vec2d<T> {
        Vec2d { x, y }
    }

    fn from(xy: (T, T)) -> Vec2d<T> {
        Vec2d { x: xy.0, y: xy.1 }
    }

}
