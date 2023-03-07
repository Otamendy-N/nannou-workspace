use nannou::{
    prelude::{map_range, Vec2, PI},
    App,
};

use crate::{boundary::Boundary, ray::Ray};

pub struct Particle {
    rays: Vec<Ray>,
}

impl Particle {
    pub fn new(number_or_rays: u16) -> Self {
        let mut rays = vec![];
        for a in 0..number_or_rays {
            let angle = map_range(a, 0, number_or_rays, 2.0 * PI, 0.0);
            let end = Vec2::new(0.0, 0.0);
            rays.push(Ray::new(Vec2::new(0.0, 0.0), end, angle));
        }
        Self { rays }
    }

    pub fn update(&mut self, app: &App, boundaries: &Vec<Boundary>) {
        for ray in &mut self.rays {
            let mouse_position = app.mouse.position();
            ray.start = mouse_position;
            let mut closest = Option::<Vec2>::None;
            for b in boundaries {
                if let Some(point) = ray.cast(b) {
                    if closest == Option::<Vec2>::None {
                        closest = Some(point);
                    } else {
                        let d1 = (ray.start - point).length();
                        let d2 = (ray.start - closest.unwrap()).length();
                        if d1 < d2 {
                            closest = Some(point);
                        }
                    }
                }
            }
            if let Some(point) = closest {
                ray.end = point;
            } else {
                ray.end = Vec2::new(0.0, 0.0);
            }
        }
    }

    pub fn show(&self, draw: &nannou::draw::Draw) {
        for ray in &self.rays {
            ray.show(&draw);
        }
    }
}
