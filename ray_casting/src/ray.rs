use nannou::prelude::*;

use crate::boundary::Boundary;

pub struct Ray {
    pub start: Vec2,
    pub end: Vec2,
    pub angle: f32,
    pub magnitude: f32,
}

impl Ray {
    pub fn new(start: Vec2, end: Vec2, angle: f32) -> Self {
        Self { start, end, angle, magnitude: 1000.0 }
    }

    pub fn cast(&self, boundary: &Boundary) -> Option<Vec2> {
        let x1 = boundary.a.x;
        let y1 = boundary.a.y;
        let x2 = boundary.b.x;
        let y2 = boundary.b.y;

        let x3 = self.start.x;
        let y3 = self.start.y;
        let x4 = self.start.x + self.angle.cos() * self.magnitude;
        let y4 = self.start.y + self.angle.sin()* self.magnitude;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denominator == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            return Some(Vec2::new(x, y));
        } else {
            return None;
        }
    }

    pub fn show(&self, draw: &nannou::draw::Draw) {
        let end = if self.end == Vec2::new(0.0, 0.0) {
            self.start + Vec2::new(self.angle.cos(), self.angle.sin()) * self.magnitude
        } else {
            self.end
        };
        draw.line()
            .start(self.start)
            .end(end)
            .weight(1.0)
            .color(WHITE);
    }
}