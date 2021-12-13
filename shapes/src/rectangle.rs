use num_traits::One;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct Rectangle<T = f32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T> {
    pub const fn new(x: T, y: T, width: T, height: T) -> Rectangle<T> {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_position_size(position: glam::XY<T>, size: glam::XY<T>) -> Rectangle<T> {
        Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        }
    }
}

impl<T> Rectangle<T>
where
    T: Copy,
{
    pub fn intersects(&self, other: &Rectangle<T>) -> bool
    where
        T: Add<Output = T> + PartialOrd,
    {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    // pub fn contains(&self, other: &Rectangle<T>) -> bool {
    //     todo!()
    // }

    pub fn contains_point(&self, point: &glam::XY<T>) -> bool
    where
        T: Add<Output = T> + PartialOrd,
    {
        self.x <= point.x
            && self.x + self.width >= point.x
            && self.y <= point.y
            && self.y + self.height >= point.y
    }

    // pub fn combine(&self, other: &Rectangle<T>) -> Rectangle<T> {
    //     todo!()
    // }

    pub fn center(&self) -> glam::XY<T>
    where
        T: One + Add<Output = T> + std::ops::Div<Output = T>,
    {
        glam::XY {
            x: self.x + (self.width / (T::one() + T::one())),
            y: self.y + (self.height / (T::one() + T::one())),
        }
    }
}
