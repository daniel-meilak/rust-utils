#![allow(dead_code)]

use std::cmp::{max, Ord};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

//================================================================
// Printing
//================================================================

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//================================================================
// Operator overlaods
//================================================================

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Mul<U, Output = T>, U: Copy> Mul<U> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: U) -> Point<T> {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Div<U, Output = T>, U: Copy + PartialEq + From<i32>> Div<U> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: U) -> Point<T> {
        if rhs == U::from(0) { panic!("Cannot divide by zero!"); }
        Point { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Point<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Point<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: MulAssign<U>, U: Copy> MulAssign<U> for Point<T> {
    fn mul_assign(&mut self, rhs: U) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: DivAssign<U>, U: Copy + PartialEq + From<i32>> DivAssign<U> for Point<T> {
    fn div_assign(&mut self, rhs: U) {
        if rhs == U::from(0) { panic!("Cannot divide by zero!"); }
        self.x /= rhs;
        self.y /= rhs;
    }
}

//================================================================
// Move in direction & get neighbours
//================================================================

impl<T: AddAssign + SubAssign + From<i32>> Point<T> {
    pub fn move_up(&mut self) {
        self.y -= T::from(1);
    }

    pub fn move_down(&mut self) {
        self.y += T::from(1);
    }

    pub fn move_left(&mut self) {
        self.x -= T::from(1);
    }

    pub fn move_right(&mut self) {
        self.x += T::from(1);
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + From<i32> + Copy> Point<T> {
    pub fn up(&self) -> Point<T> {
        Point { x: self.x, y: self.y - T::from(1) }
    }

    pub fn down(&self) -> Point<T> {
        Point { x: self.x, y: self.y + T::from(1) }
    }
    
    pub fn left(&self) -> Point<T> {
        Point { x: self.x - T::from(1), y: self.y }
    }
    
    pub fn right(&self) -> Point<T> {
        Point { x: self.x + T::from(1), y: self.y }
    }    
}

//================================================================
// Other related functions
//================================================================

pub trait Abs {
    fn abs(self) -> Self;
}

macro_rules! define_abs {
    ($($T:ty),*) => {
        $(
            impl Abs for $T {
                fn abs(self) -> Self {
                    if self < 0 { -self }
                    else        {  self }
                }
            }
        )*
    }
}

define_abs!(i32, i64, i128);

// cardinal distance (diagonal movement is longer)
pub fn manhattan<T: Sub<Output = T> + Add<Output = T> + Copy + Abs>(lhs: &Point<T>, rhs: &Point<T>) -> T {
    (rhs.x - lhs.x).abs() + (rhs.y - lhs.y).abs()
}

pub fn chebyshev<T: Sub<Output = T> + Add<Output = T> + Copy + Ord + Abs>(lhs: &Point<T>, rhs: &Point<T>) -> T {
    max((rhs.x - lhs.x).abs(), (rhs.y - lhs.y).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Much of these tests are really only checking that I've understood
    // how to implement thigns correctly. They're also just to try using
    // the struct, and to get used to writing tests in rust.

    #[test]
    fn point_print() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.to_string(), "(1, 2)");
    }

    #[test]
    fn point_equality() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 1, y: 2 };
        let p3 = Point { x: 2, y: 2 };
        let p4 = Point { x: 2, y: 1 };
        
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, p4);
    }

    #[test]
    fn point_order() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 2, y: 1 };
        let p3 = Point { x: 1, y: 3 };
        let p4 = Point { x: 1, y: 1 };
        let p5 = Point { x: 0, y: 1 };

        assert!(p1 < p2);
        assert!(p1 < p3);
        assert!(p1 > p4);
        assert!(p1 > p5);
    }

    #[test]
    fn point_add() {
        let mut p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = Point { x: 4, y: 6 };
        
        assert_eq!(p1 + p2, p3);
        assert_eq!(p1 + p2, p2 + p1);
        
        p1 += p2; 
        assert_eq!(p1, p3);
    }

    #[test]
    fn point_subtract() {
        let mut p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = Point { x: -2, y: -2 };

        assert_eq!(p1 - p2, p3);
        assert_ne!(p1 - p2, p2 - p1);

        p1 -= p2;
        assert_eq!(p1, p3);
    }

    #[test]
    fn point_multiply() {
        let mut p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 2, y: 4 };
        let n = 2;

        assert_eq!(p1 * n, p2);

        p1 *= n;
        assert_eq!(p1, p2);
    }

    #[test]
    fn point_divide() {
        let mut p1 = Point { x: 3, y: 9 };
        let p2 = Point { x: 1, y: 3 };
        let n = 3;

        assert_eq!(p1 / n, p2);

        p1 /= n;
        assert_eq!(p1, p2);
    }

    #[test]
    fn point_up() {
        let mut p1 = Point { x: 1, y: 2 };
        let up = Point { x: 0, y: -1 };
        let result = p1 + up;

        assert_eq!(p1.up() , result);

        p1.move_up();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_down() {
        let mut p1 = Point { x: 1, y: 2 };
        let down = Point { x: 0, y: 1 };
        let result = p1 + down;

        assert_eq!(p1.down() , result);

        p1.move_down();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_left() {
        let mut p1 = Point { x: 1, y: 2 };
        let left = Point { x: -1, y: 0 };
        let result = p1 + left;

        assert_eq!(p1.left() , result);

        p1.move_left();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_right() {
        let mut p1 = Point { x: 1, y: 2 };
        let right = Point { x: 1, y: 0 };
        let result = p1 + right;

        assert_eq!(p1.right() , result);

        p1.move_right();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_manhattan() {
        let p1 = Point { x: 1, y: 2};
        let p2 = Point { x: 3, y: 4};
        let p3 = Point { x: -3, y: -4 };

        assert_eq!(manhattan(&p1, &p2), 4);
        assert_eq!(manhattan(&p1, &p3), 10);
    }

    #[test]
    fn point_chebyshev() {
        let p1 = Point { x: 1, y: 2};
        let p2 = Point { x: 3, y: 5};
        let p3 = Point { x: -3, y: -5 };

        assert_eq!(chebyshev(&p1, &p2), 3);
        assert_eq!(chebyshev(&p1, &p3), 7);
    }
}
