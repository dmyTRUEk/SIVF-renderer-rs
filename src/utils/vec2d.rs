//! Mathematical Vector (2 dimensional)

use std::convert::TryFrom;
use std::ops::{Neg, Sub, Add, Div, Mul};

use crate::utils::sizes::Sizes;



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d<T: Copy> { pub x: T, pub y: T }

impl<T: Copy> Vec2d<T> {

    pub fn new(x: T, y: T) -> Self {
        Vec2d { x, y }
    }

    pub fn from_tuple((x, y): (T, T)) -> Vec2d<T> {
        Vec2d { x, y }
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

    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn is_inside_circle(&self, radius2: f64) -> bool {
        self.len2() < radius2
    }

    pub fn is_inside_rectangle(&self, vec_min: Self, vec_max: Self) -> bool {
        (vec_min.x <= self.x && self.x <= vec_max.x) && (vec_min.y <= self.y && self.y <= vec_max.y)
    }

    pub fn is_inside_triangle(&self, point1: Self, point2: Self, point3: Self) -> bool {
        fn triangle_sign(p1: Vec2d<f64>, p2: Vec2d<f64>, p3: Vec2d<f64>) -> f64 {
            (p1.x-p3.x)*(p2.y-p3.y) - (p2.x-p3.x)*(p1.y-p3.y)
        }
        let d1 = triangle_sign(*self, point1, point2);
        let d2 = triangle_sign(*self, point2, point3);
        let d3 = triangle_sign(*self, point3, point1);
        !((d1 < 0.0 || d2 < 0.0 || d3 < 0.0) && (d1 > 0.0 || d2 > 0.0 || d3 > 0.0))
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

impl<T: Copy + Mul<f64, Output=T>> Mul<f64> for Vec2d<T> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
