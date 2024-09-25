use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq)]
struct Complex {
    real: i32,
    imaginary: i32,
}

impl Complex {
    fn new(real: i32, imaginary: i32) -> Self {
        Self { real, imaginary }
    }

    fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self {
            real: 0,
            imaginary: 1,
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.real;
        let b = self.imaginary;
        let c = rhs.real;
        let d = rhs.imaginary;
        Self {
            real: a * c - b * d,
            imaginary: a * d + b * c,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;

    #[test]
    fn test_add() {
        let a = Complex::new(1, 1);
        let b = Complex::new(3, 4);
        let c = a + b;
        println!("{c}");
        assert!(c == Complex::new(4, 5))
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(3, -1);
        let b = Complex::new(2, 3);
        let c = a - b;
        println!("{c}");
        assert!(c == Complex::new(1, -4))
    }
    #[test]
    fn test_mul() {
        let a = Complex::new(0, 2);
        let b = Complex::new(0, 4);
        let c = a * b;
        println!("{c}");
        assert!(c == Complex::new(-8, 0))
    }
}
