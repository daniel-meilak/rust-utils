use std::fmt;
use std::ops;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn point_add_subtract() {
        let mut p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        
        assert_eq!(p1 + p2, Point { x: 4, y: 6 });
        assert_eq!(p1 + p2, p2 + p1);
        assert_eq!(p1 - p2, Point { x: -2, y: -2 });
        assert_ne!(p1 - p2, p2 - p1);

        p1 += p2; 
        assert_eq!(p1, Point { x: 4, y: 6 });

        p1 -= p2;
        assert_eq!(p1, Point { x: 1, y: 2 });
    }
}
