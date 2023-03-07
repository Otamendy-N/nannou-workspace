use nannou::prelude::*;

struct Model {
    end: Vec2,
    angle: f32,
    velocity: f32,
    acceleration: f32,
    gravity: f32,
    start: Vec2,
}

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {
        end: Vec2::new(0.0, -300.0),
        start: Vec2::new(0.0, 0.0),
        angle: PI / 3.0,
        velocity: 0.0,
        acceleration: 0.0,
        gravity: 1.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let force = model.gravity * model.angle.sin() / model.end.length();
    let len = model.end.length();
    let mass = 3.0;
    model.acceleration = force / mass;
    model.velocity += model.acceleration;
    model.angle += model.velocity;
    model.end.x = len * model.angle.sin() + model.start.x;
    model.end.y = len * model.angle.cos() - model.start.y;
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.line()
        .start(model.start)
        .end(model.end)
        .weight(1.0)
        .color(WHITE);

    draw.ellipse()
        .x_y(model.end.x, model.end.y)
        .w_h(100.0, 100.0)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
