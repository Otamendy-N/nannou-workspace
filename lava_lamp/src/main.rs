use nannou::{
    prelude::*,
    rand::{self, Rng},
};

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 800.0;
const RESOLUTION: f32 = 50.0;
const FIELD_LENGTH: usize = ((WIDTH / RESOLUTION) as usize) * ((HEIGHT / RESOLUTION) as usize);

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    field: [f32; FIELD_LENGTH],
}

fn model(_app: &App) -> Model {
    let mut field = [0.0; FIELD_LENGTH];
    let mut rng = rand::thread_rng();
    for i in 0..field.len() {
        field[i] = rng.gen::<f32>();
    }
    Model { field }
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().x_y(-WIDTH / 2.0, -HEIGHT / 2.0);
    draw.background().color(BLACK);
    for i in 0..((WIDTH / RESOLUTION) as usize) {
        for j in 0..((HEIGHT / RESOLUTION) as usize) {
            let x = i as f32 * RESOLUTION;
            let y = j as f32 * RESOLUTION;

            let a = model.field[i * j];
            let b = model.field[(i+1) * j];
            let c = model.field[(i+1) * (j+1)];
            let d = model.field[i * (j+1)];

            let value = a as u8 * 8 + b as u8 * 4 + c as u8 * 2 + d as u8;

            let value = model.field[i * j];
            draw_lines(value, x, y, &draw);
            draw.ellipse().x_y(x, y).radius(10.0).rgba(
                255.0,
                255.0,
                255.0,
                value
            );
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_lines(value: f32, x: f32, y: f32, draw: &Draw) {
    let a = Point2::new(x + RESOLUTION*0.5, y);
    let b = Point2::new(x + RESOLUTION, y + RESOLUTION*0.5);
    let c = Point2::new(x + RESOLUTION*0.5, y + RESOLUTION);
    let d = Point2::new(x, y + RESOLUTION*0.5);



}

fn line(a: Point2, b: Point2, draw: &Draw) {
    draw.line().points(a, b).color(WHITE);
}