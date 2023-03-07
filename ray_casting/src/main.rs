mod boundary;
mod ray;
mod particle;

use boundary::Boundary;
use nannou::prelude::*;
use particle::Particle;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    boundaries: Vec<Boundary>,
    particle: Particle,
}

fn model(_app: &App) -> Model {
    let boundary_a = Boundary::new(Vec2::new(-100.0, 100.0), Vec2::new(-100.0, -100.0));
    let boundary_b = Boundary::new(Vec2::new(-100.0, 100.0), Vec2::new(100.0, 100.0));
    let boundary_c = Boundary::new(Vec2::new(100.0, 100.0), Vec2::new(100.0, -100.0));

    let particle = Particle::new(360);
    Model {
        boundaries: vec![boundary_a, boundary_b, boundary_c],
        particle,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.particle.update(app, &model.boundaries);
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for boundary in &model.boundaries {
        boundary.show(&draw);
    }
    model.particle.show(&draw);
    draw.to_frame(app, &frame).unwrap();
}
