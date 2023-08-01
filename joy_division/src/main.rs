use nannou::prelude::*;

const SIZE: usize = 320;
const LINE_WIDTH: f32 = 1.2;
const WIDTH: u32 = 320;
const HEIGHT: u32 = 320;
const STEP: usize = 10;

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

    let mut lines = Vec::<Vec<Vec2>>::new();

    for i in (STEP..=SIZE - STEP).step_by(STEP) {
        let mut line = Vec::<Vec2>::new();

        for j in (STEP..=SIZE - STEP).step_by(STEP) {
            let distance_to_center = j - SIZE / 2;
            let variance = (SIZE / 2 - 50 - distance_to_center).max(0) as isize;
            let random = random::<isize>();
            let random = random * variance / 2 * -1;
            let point = pt2(j as f32, i as f32 + random as f32);
            line.push(point);
        }
        lines.push(line);
    }

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            draw.line()
                .start(pt2(lines[i][0].x, lines[i][0].y))
                .end(pt2(lines[i][j].x, lines[i][j].y))
                .weight(LINE_WIDTH)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
