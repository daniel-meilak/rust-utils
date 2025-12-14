use num_traits::{One, Signed, Unsigned, Zero};
use std::cmp::{max, Ord};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Default> Default for Point<T> {
    fn default() -> Self {
        Point::new(T::default(), T::default())
    }
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
// Operator overloads
//================================================================

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Mul<U, Output = T>, U: Copy> Mul<U> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: U) -> Point<T> {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Div<U, Output = T>, U: Copy + PartialEq + Zero> Div<U> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: U) -> Point<T> {
        if rhs == U::zero() {
            panic!("Cannot divide by zero!");
        }

        Point::new(self.x / rhs, self.y / rhs)
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

impl<T: DivAssign<U>, U: Copy + PartialEq + Zero> DivAssign<U> for Point<T> {
    fn div_assign(&mut self, rhs: U) {
        if rhs == U::zero() {
            panic!("Cannot divide by zero!");
        }

        self.x /= rhs;
        self.y /= rhs;
    }
}

//================================================================
// Move in direction & get neighbours
//================================================================

impl<T: AddAssign + SubAssign + One> Point<T> {
    pub fn move_up(&mut self) {
        self.y -= T::one();
    }

    pub fn move_down(&mut self) {
        self.y += T::one();
    }

    pub fn move_left(&mut self) {
        self.x -= T::one();
    }

    pub fn move_right(&mut self) {
        self.x += T::one();
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + One + Copy> Point<T> {
    pub fn up(&self) -> Point<T> {
        Point::new(self.x, self.y - T::one())
    }

    pub fn down(&self) -> Point<T> {
        Point::new(self.x, self.y + T::one())
    }

    pub fn left(&self) -> Point<T> {
        Point::new(self.x - T::one(), self.y)
    }

    pub fn right(&self) -> Point<T> {
        Point::new(self.x + T::one(), self.y)
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + One + Copy> Point<T> {
    pub fn neighbors(&self) -> [Point<T>; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }
}

//================================================================
// Other related functions
//================================================================

// cardinal distance (diagonal movement is longer)
pub fn manhattan<T>(lhs: &Point<T>, rhs: &Point<T>) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Copy + Signed,
{
    (rhs.x - lhs.x).abs() + (rhs.y - lhs.y).abs()
}

pub fn manhattan_unsigned<T>(lhs: &Point<T>, rhs: &Point<T>) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Copy + Ord + Unsigned,
{
    let dx = if rhs.x > lhs.x { rhs.x - lhs.x } else { lhs.x - rhs.x };
    let dy = if rhs.y > lhs.y { rhs.y - lhs.y } else { lhs.y - rhs.y };

    dx + dy
}

pub fn chebyshev<T>(lhs: &Point<T>, rhs: &Point<T>) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Copy + Ord + Signed,
{
    max((rhs.x - lhs.x).abs(), (rhs.y - lhs.y).abs())
}

pub fn chebyshev_unsigned<T>(lhs: &Point<T>, rhs: &Point<T>) -> T
where
    T: Sub<Output = T> + Copy + Ord + Unsigned,
{
    let dx = if rhs.x > lhs.x { rhs.x - lhs.x } else { lhs.x - rhs.x };
    let dy = if rhs.y > lhs.y { rhs.y - lhs.y } else { lhs.y - rhs.y };

    max(dx, dy)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Much of these tests are really only checking that I've understood
    // how to implement things correctly. They're also just to try using
    // the struct, and to get used to writing tests in rust.

    #[test]
    fn point_print() {
        let a = Point::new(1u32, 2u32);
        let b = Point::new(1, 2);
        let c = Point::new(1.0, 2.0);

        assert_eq!(a.to_string(), "(1, 2)");
        assert_eq!(b.to_string(), "(1, 2)");
        assert_eq!(c.to_string(), "(1, 2)");
    }

    #[test]
    fn point_equality() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(1, 2);
        let p3 = Point::new(2, 2);
        let p4 = Point::new(2, 1);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, p4);
    }

    #[test]
    fn point_order() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(2, 1);
        let p3 = Point::new(1, 3);
        let p4 = Point::new(1, 1);
        let p5 = Point::new(0, 1);

        assert!(p1 < p2);
        assert!(p1 < p3);
        assert!(p1 > p4);
        assert!(p1 > p5);
    }

    #[test]
    fn point_add() {
        let mut p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = Point::new(4, 6);

        assert_eq!(p1 + p2, p3);
        assert_eq!(p1 + p2, p2 + p1);

        p1 += p2;
        assert_eq!(p1, p3);
    }

    #[test]
    fn point_subtract() {
        let mut p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = Point::new(-2, -2);

        assert_eq!(p1 - p2, p3);
        assert_ne!(p1 - p2, p2 - p1);

        p1 -= p2;
        assert_eq!(p1, p3);
    }

    #[test]
    fn point_multiply() {
        let mut p1 = Point::new(1, 2);
        let p2 = Point::new(2, 4);
        let n = 2;

        assert_eq!(p1 * n, p2);

        p1 *= n;
        assert_eq!(p1, p2);
    }

    #[test]
    fn point_divide() {
        let mut p1 = Point::new(3, 9);
        let p2 = Point::new(1, 3);
        let n = 3;

        assert_eq!(p1 / n, p2);

        p1 /= n;
        assert_eq!(p1, p2);
    }

    #[test]
    fn point_up() {
        let mut p1 = Point::new(1, 2);
        let up = Point::new(0, -1);
        let result = p1 + up;

        assert_eq!(p1.up(), result);

        p1.move_up();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_down() {
        let mut p1 = Point::new(1, 2);
        let down = Point::new(0, 1);
        let result = p1 + down;

        assert_eq!(p1.down(), result);

        p1.move_down();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_left() {
        let mut p1 = Point::new(1, 2);
        let left = Point::new(-1, 0);
        let result = p1 + left;

        assert_eq!(p1.left(), result);

        p1.move_left();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_right() {
        let mut p1 = Point::new(1, 2);
        let right = Point::new(1, 0);
        let result = p1 + right;

        assert_eq!(p1.right(), result);

        p1.move_right();
        assert_eq!(p1, result);
    }

    #[test]
    fn point_manhattan() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = Point::new(-3, -4);

        assert_eq!(manhattan(&p1, &p2), 4);
        assert_eq!(manhattan(&p1, &p3), 10);
    }

    #[test]
    fn point_manhattan_unsigned() {
        let p1 = Point::new(1u32, 2u32);
        let p2 = Point::new(3u32, 4u32);

        assert_eq!(manhattan_unsigned(&p1, &p2), 4);
        assert_eq!(manhattan_unsigned(&p2, &p1), 4);
    }

    #[test]
    fn point_chebyshev() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 5);
        let p3 = Point::new(-3, -5);

        assert_eq!(chebyshev(&p1, &p2), 3);
        assert_eq!(chebyshev(&p1, &p3), 7);
    }

    #[test]
    fn point_chebyshev_unsigned() {
        let p1 = Point::new(1u32, 2u32);
        let p2 = Point::new(3u32, 5u32);

        assert_eq!(chebyshev_unsigned(&p1, &p2), 3);
        assert_eq!(chebyshev_unsigned(&p2, &p1), 3);
    }
}
