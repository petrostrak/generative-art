use nannou::prelude::*;

// const LINE_WIDTH: f32 = 2.0;
const MIN_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 120.0;
const N_CIRCLES: usize = 100;
// const CREATE_CIRCLE_ATTEMPTS: usize = 10;

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
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    let points = (0..=N_CIRCLES).map(|_| {
        let x = random_range(window.left(), window.right());
        let y = random_range(window.top(), window.bottom());
        let radius = random_range(MIN_RADIUS, MAX_RADIUS);

        (pt2(x, y), radius, WHITE)
    });

    for (point, radius, color) in points {
        draw.ellipse()
            .w_h(radius, radius)
            .x_y(point.x, point.y)
            .color(color);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
