use num_traits::One;
use num_traits::Zero;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::MulAssign;

pub type Rect = Rectangle<f32>;
pub type IRect = Rectangle<i32>;

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

    pub fn new_with_size(width: T, height: T) -> Rectangle<T>
    where
        T: Zero,
    {
        Rectangle {
            x: T::zero(),
            y: T::zero(),
            width,
            height,
        }
    }

    pub fn translate(&mut self, offset: glam::XY<T>)
    where
        T: AddAssign,
    {
        self.x += offset.x;
        self.y += offset.y;
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn move_to(&mut self, destination: glam::XY<T>)
    where
        T: AddAssign,
    {
        self.x = destination.x;
        self.y = destination.y;
    }

    /// Scales the `Rect` by a factor of (sx, sy),
    /// growing towards the bottom-left
    pub fn scale(&mut self, sx: T, sy: T)
    where
        T: MulAssign,
    {
        self.width *= sx;
        self.height *= sy;
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

    pub fn right(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.x + self.width
    }

    pub fn bottom(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.y + self.height
    }

    pub fn translated(&self, offset: glam::XY<T>) -> Rectangle<T>
    where
        T: AddAssign,
    {
        let mut rectangle = self.clone();
        rectangle.translate(offset);
        rectangle
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn moved_to(&self, destination: glam::XY<T>) -> Rectangle<T>
    where
        T: AddAssign,
    {
        let mut rectangle = self.clone();
        rectangle.move_to(destination);
        rectangle
    }

    /// Scales the `Rect` by a factor of (sx, sy),
    /// growing towards the bottom-left
    pub fn scaled(&self, sx: T, sy: T) -> Rectangle<T>
    where
        T: MulAssign,
    {
        let mut rectangle = self.clone();
        rectangle.scale(sx, sy);
        rectangle
    }
}
