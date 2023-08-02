use nannou::prelude::*;

const SIZE: usize = 500;
const LINE_WIDTH: f32 = 2.0;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const STEP: usize = 15;

struct Model;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw().x_y(-window.w() * 0.5, -window.h() * 0.5);
    draw.background().color(SNOW);

    let mut lines = vec![];
    for i in (STEP..=SIZE - STEP).step_by(STEP) {
        let mut line = vec![];
        for j in (STEP..=SIZE - STEP).step_by(STEP) {
            let distance_to_center = (j as f32 - SIZE as f32 / 2.0).abs();
            let variance = (SIZE as f32 / 2.0 - 50.0 - distance_to_center).max(0.0);
            let random = random::<f32>();
            let random = random * variance / 2.0;
            let point = pt2(j as f32, i as f32 + random);
            line.push(point);
        }
        lines.push(line);
    }

    for i in 0..lines.len() - 5 {
        let vertices = (0..lines[i].len() - 1).map(|j| {
            let xc = (lines[i][j].x + lines[i][j + 1].x) / 2.0;
            let yc = (lines[i][j].y + lines[i][j + 1].y) / 2.0;

            let point = pt2(xc, yc);

            (point, BLACK)
        });

        draw.polyline().weight(LINE_WIDTH).points_colored(vertices);
    }

    draw.to_frame(app, &frame).unwrap();
}
