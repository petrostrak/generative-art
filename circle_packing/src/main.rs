use nannou::prelude::*;

const LINE_WIDTH: f32 = 2.0;
const MIN_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 5.0;
const N_CIRCLES: usize = 1500;
// const CREATE_CIRCLE_ATTEMPTS: usize = 500;

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
    draw.background().color(WHITE);

    for _ in 0..=N_CIRCLES {
        let x = random_range(window.left(), window.right());
        let y = random_range(window.top(), window.bottom());
        let radius = random_range(MIN_RADIUS, MAX_RADIUS);

        let line_points = (0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x_ = x + radian.sin() + radius;
            let y_ = y + radian.cos() + radius;
            (pt2(x_, y_), BLACK)
        });

        draw.ellipse().w_h(radius, radius).x_y(x, y).color(GRAY);

        draw.polyline()
            .weight(LINE_WIDTH)
            .points_colored(line_points);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
