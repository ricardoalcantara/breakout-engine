#[derive(Debug, Copy, Clone, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_position_size(position: glam::Vec2, size: glam::Vec2) -> Rect {
        Rect {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        }
    }

    pub fn new_with_size(width: f32, height: f32) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

    pub fn translate(&mut self, offset: glam::Vec2) {
        self.x += offset.x;
        self.y += offset.y;
    }

    pub fn move_to(&mut self, destination: glam::Vec2) {
        self.x = destination.x;
        self.y = destination.y;
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.width *= sx;
        self.height *= sy;
    }
}

impl Rect {
    pub fn size(&self) -> glam::Vec2 {
        glam::vec2(self.width, self.height)
    }

    pub fn position(&self) -> glam::Vec2 {
        glam::vec2(self.x, self.y)
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn contains(&self, _other: &Rect) -> bool {
        self.x <= _other.x
            && self.x + self.width >= _other.right()
            && self.y <= _other.y
            && self.y + self.height >= _other.bottom()
    }

    pub fn contains_point(&self, point: &glam::Vec2) -> bool {
        self.x <= point.x
            && self.x + self.width >= point.x
            && self.y <= point.y
            && self.y + self.height >= point.y
    }

    // pub fn combine(&self, other: &Rect) -> Rect {
    //     todo!()
    // }

    pub fn center(&self) -> glam::Vec2 {
        glam::vec2(self.x + (self.width / 2.0), self.y + (self.height / 2.0))
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn translated(&self, offset: glam::Vec2) -> Rect {
        let mut rectangle = self.clone();
        rectangle.translate(offset);
        rectangle
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn moved_to(&self, destination: glam::Vec2) -> Rect {
        let mut rectangle = self.clone();
        rectangle.move_to(destination);
        rectangle
    }

    /// Scales the `Rect` by a factor of (sx, sy),
    /// growing towards the bottom-left
    pub fn scaled(&self, sx: f32, sy: f32) -> Rect {
        let mut rectangle = self.clone();
        rectangle.scale(sx, sy);
        rectangle
    }
}
