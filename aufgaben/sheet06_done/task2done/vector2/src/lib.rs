extern crate num_traits;

mod tests;

use std::ops::Add;
use std::ops::Mul;
use num_traits::{Zero, One};

#[derive(PartialEq, Debug)]
struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    fn new(x: T, y: T) -> Self {
        Vector2 {
            x: x,
            y: y,
        }
    }
}

impl<T: Zero> Vector2<T> {
    fn origin() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T> Vector2<T> 
	where T: Zero + One,
{
    fn unit_x() -> Self {
        Self::new(T::one(), T::zero())
    }

    fn unit_y() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl <T, U> Add<Vector2<U>> for Vector2<T>
    where T: Add<U>,
{
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<U>) -> Self::Output{
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T, U> Mul<U> for Vector2<T>
    where T: Mul<U>,
          U: Clone,
{
    type Output = Vector2<T::Output>;

    fn mul(self, other: U) -> Self::Output{ 
        Vector2::new(self.x * other.clone(), self.y * other)
    }
}
