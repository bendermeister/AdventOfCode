use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn as_coords(&self) -> Option<(usize, usize)> {
        let x = match self.x.try_into() {
            Ok(x) => x,
            Err(_) => return None,
        };
        let y = match self.y.try_into() {
            Ok(y) => y,
            Err(_) => return None,
        };
        Some((x, y))
    }

    pub fn as_coords_with_bound(&self, width: usize, height: usize) -> Option<(usize, usize)> {
        let (x, y) = match self.as_coords() {
            Some(n) => n,
            None => return None,
        };
        if x >= width || y >= height {
            return None;
        }
        Some((x, y))
    }

    pub fn scale(&self, factor: isize) -> Self {
        Self {x: self.x * factor, y: self.y * factor}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Point::new(34, 35);
        let b = Point::new(35, 34);
        let c = a + b;
        let expected = Point::new(69, 69);
        assert_eq!(expected, c);
    }

    #[test]
    fn test_as_coords() {
        let a = Point::new(34, 35);
        a.as_coords().unwrap();
    }

    #[test]
    fn test_as_coords_fail0() {
        let a = Point::new(-1, 35);
        assert!(a.as_coords().is_none());
    }

    #[test]
    fn test_as_coords_fail1() {
        let a = Point::new(0, -1);
        assert!(a.as_coords().is_none());
    }

    #[test]
    fn test_as_coords_fail2() {
        let a = Point::new(-2, -35);
        assert!(a.as_coords().is_none());
    }

    #[test]
    fn test_as_coords_with_bound() {
        let a = Point::new(34, 35);
        a.as_coords_with_bound(100, 100).unwrap();
    }

    #[test]
    fn test_as_coords_with_bound_fail0() {
        let a = Point::new(1, 35);
        assert!(a.as_coords_with_bound(0, 0).is_none());
    }

    #[test]
    fn test_scale() {
        let a = Point::new(1, 1).scale(3);
        let expected = Point::new(3, 3);
        assert_eq!(expected, a);
    }
}
