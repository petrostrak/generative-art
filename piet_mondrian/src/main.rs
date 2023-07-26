use nannou::prelude::*;

struct Model;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().view(view).build().unwrap();
    Model
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
