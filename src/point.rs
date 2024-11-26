use std::cmp;
use std::fmt;
use std::ops;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// pub mod dir {
//     use super::Point;

//     pub const UP   : Point = Point { x:  0, y: -1 };
//     pub const DOWN : Point = Point { x:  0, y:  1 };
//     pub const LEFT : Point = Point { x: -1, y:  0 };
//     pub const RIGHT: Point = Point { x:  1, y:  0 };
// }

//================================================================
// Printing
//================================================================

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//================================================================
// Operator overlaods
//================================================================

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        if rhs == 0 { panic!("Cannot divide by zero!"); }
        Point { x: self.x / rhs, y: self.y / rhs }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Point { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = Point { x: self.x - rhs.x, y: self.y - rhs.y };
    }
}

impl ops::MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        *self = Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::DivAssign<i32> for Point {
    fn div_assign(&mut self, rhs: i32) {
        if rhs == 0 { panic!("Cannot divide by zero!"); }
        *self = Point { x: self.x / rhs, y: self.y / rhs }
    }
}

//================================================================
// Move in direction & get neighbours
//================================================================

#[allow(dead_code)]
impl Point {
    fn move_up(&mut self) {
        self.y -= 1;
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn up(&self) -> Point {
        Point { x: self.x, y: self.y - 1 }
    }

    fn down(&self) -> Point {
        Point { x: self.x, y: self.y + 1 }
    }
    
    fn left(&self) -> Point {
        Point { x: self.x - 1, y: self.y }
    }
    
    fn right(&self) -> Point {
        Point { x: self.x + 1, y: self.y }
    }    
}

//================================================================
// Other related functions
//================================================================

#[allow(dead_code)]
impl Point {
    // cardinal distance (diagonal movement is longer)
    fn manhattan(lhs: &Point, rhs: &Point) -> i32 {
        (rhs.x - lhs.x).abs() + (rhs.y - lhs.y).abs()
    }

    // diagonal distance equal to cardinal
    fn chebyshev(lhs: &Point, rhs: &Point) -> i32 {
        cmp::max((rhs.x - lhs.x).abs(), (rhs.y - lhs.y).abs())
    }
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


}
