//! Mathematical Vector (2 dimensional)

use std::convert::TryFrom;

use crate::utils::sizes::Sizes;
use std::ops::{Neg, Sub, Add, Mul, Div};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d<T: Copy> { pub x: T, pub y: T }

impl<T: Copy> Vec2d<T> {

    pub fn new(x: T, y: T) -> Self {
        Vec2d { x, y }
    }

    pub fn from_tuple(xy: (T, T)) -> Vec2d<T> {
        Vec2d { x: xy.0, y: xy.1 }
    }

    pub fn to_vec2d<R: Copy + TryFrom<T>>(&self) -> Vec2d<R> {
        Vec2d {
            x: R::try_from(self.x).ok().unwrap(),
            y: R::try_from(self.y).ok().unwrap(),
        }
    }

    pub fn to_sizes<R: Copy + TryFrom<T>>(&self) -> Sizes<R> {
        Sizes {
            w: R::try_from(self.x).ok().unwrap(),
            h: R::try_from(self.y).ok().unwrap(),
        }
    }

}

impl Vec2d<f64> {

    pub fn len2(&self) -> f64 {
        // self.x.powi(2) + self.y.powi(2)
        self.x*self.x + self.y*self.y
    }

    pub fn is_inside_circle(&self, radius2: f64) -> bool {
        self.len2() < radius2
    }

    pub fn is_inside_rectangle(&self, vec_min: Self, vec_max: Self) -> bool {
        (vec_min.x <= self.x && self.x <= vec_max.x) && (vec_min.y <= self.y && self.y <= vec_max.y)
    }

    pub fn is_inside_triangle(&self, p1: Self, p2: Self, p3: Self) -> bool {
        todo!()
    }

}

impl<T: Copy + Neg<Output=T>> Neg for Vec2d<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec2d {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Copy + Add<Output=T>> Add for Vec2d<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Copy + Sub<Output=T>> Sub for Vec2d<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + Div<f64, Output=T>> Div<f64> for Vec2d<T> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
