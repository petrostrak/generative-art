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

    for i in (STEP..=SIZE - STEP).step_by(STEP).skip(5) {
        let vertices = (STEP..=SIZE - STEP).step_by(STEP).map(|j| {
            let distance_to_center = (j as f32 - SIZE as f32 / 2.0).abs();
            let variance = (SIZE as f32 / 2.0 - 50.0 - distance_to_center).max(0.0);
            let random = random::<f32>();
            let random = random * variance / 2.0 * -1.0;
            let point = pt2(j as f32, i as f32 + random);

            println!("POINT OF LINE_{:?}_{:?}", i, point);
            (pt2(j as f32, i as f32 + random), BLACK)
        });

        draw.polyline().weight(LINE_WIDTH).points_colored(vertices);
    }

    draw.to_frame(app, &frame).unwrap();
}
