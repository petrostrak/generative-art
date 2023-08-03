use nannou::prelude::*;
use num_complex::Complex;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 || app.keys.down.contains(&Key::R) {
        draw.background().color(SNOW);
    }

    for i in 0..600 {
        for j in 0..400 {
            let re = (i as f32) / 200.0 - 2.0;
            let im = (j as f32) / 200.0 - 1.0;
            let c = Complex::new(re, im);
            if check(c) {
                draw.rect()
                    .w_h(1.0, 1.0)
                    .x_y(re * 200.0 + 150.0, im * 200.0)
                    .color(BLACK);
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn check(c: Complex<f32>) -> bool {
    let mut z = Complex::new(0.0, 0.0);
    for _i in 0..30 {
        z = z * z + c;
        if z.norm() > 2.0 {
            return false;
        }
    }
    return true;
}
