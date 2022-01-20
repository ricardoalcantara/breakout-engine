use super::Constraints;

pub(crate) struct Screen {
    x: Constraints,
    y: Constraints,
}

impl Screen {
    pub(crate) fn new(x: Constraints, y: Constraints) -> Screen {
        Screen { x, y }
    }

    pub(crate) fn position(&self) -> glam::Vec2 {
        let x = match self.x {
            Constraints::Pixel(x) => x,
            Constraints::Auto => 0,
            _ => 0,
        };
        let y = match self.y {
            Constraints::Pixel(y) => y,
            Constraints::Auto => 0,
            _ => 0,
        };

        glam::vec2(x as f32, y as f32)
    }
}
