#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a * *b)
            .fold(T::zero(), |acc, x| acc + x);

        Some(result)
    }
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Vector<T>) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(result))
    }
}

use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Copy
{
    fn zero() -> Self;
    fn one() -> Self;
}
impl Scalar for u32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}
