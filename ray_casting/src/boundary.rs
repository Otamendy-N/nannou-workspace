use nannou::prelude::*;

pub struct Boundary {
    pub a: Vec2,
    pub b: Vec2,
}

impl Boundary {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    pub fn show(&self, draw: &nannou::draw::Draw) {
        draw.line()
            .start(self.a)
            .end(self.b)
            .weight(1.0)
            .color(WHITE);
    }
}