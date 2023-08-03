use nannou::{prelude::*, rand};

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const LINE_WIDTH: f32 = 5.0;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIDTH, HEIGHT)
        .run();
}

struct Model {
    diagonals: Vec<Diagonal>,
}
struct Diagonal {
    start: Vec2,
    end: Vec2,
}

impl Diagonal {
    fn random_from_rect(rect: Rect) -> Self {
        if rand::random() {
            Diagonal {
                start: rect.bottom_right(),
                end: rect.top_left(),
            }
        } else {
            Diagonal {
                start: rect.bottom_left(),
                end: rect.top_right(),
            }
        }
    }
}

fn model(app: &App) -> Model {
    let window_rect = app.window_rect();
    let mut diagonals: Vec<Diagonal> = Vec::new();
    for rect in window_rect.subdivisions_iter() {
        for rect in rect.subdivisions_iter() {
            for rect in rect.subdivisions_iter() {
                diagonals.push(Diagonal::random_from_rect(rect));
            }
        }
    }
    Model { diagonals }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(SNOW);

    let draw = app.draw();

    let diagonals: &Vec<Diagonal> = &model.diagonals;

    for diagonal in diagonals {
        draw.line()
            .start(diagonal.start)
            .end(diagonal.end)
            .weight(LINE_WIDTH)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
