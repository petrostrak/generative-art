use nannou::prelude::*;
use num_complex::Complex;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .key_pressed(key_pressed)
        // .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 || app.keys.down.contains(&Key::R) {
        draw.background().color(BLACK);
    }

    for i in 0..100 {
        for j in 0..100 {
            draw.rect()
                .w_h(7.0, 7.0)
                .x_y(10.0 * i as f32, 10.0 * j as f32)
                .color(WHITE);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn my_sin(a: f32, x: f32) -> f32 {
    return a * x.sin();
}

fn my_cos(a: f32, x: f32) -> f32 {
    return a * x.cos();
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => app
            .main_window()
            .capture_frame(app.exe_name().unwrap() + &format!("{:03}", app.time) + ".png"),
        _other_key => {}
    }
}
