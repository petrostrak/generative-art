use nannou::prelude::*;

const LINE_WIDTH: f32 = 6.0;
const N_RECTANGLES: usize = 14;

struct Model;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().size(400, 600).view(view).build().unwrap();
    Model
}

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();
    let size_w = window.w().try_into().map(|f: f32| f - LINE_WIDTH).unwrap();
    let size_h = window.h().try_into().map(|f: f32| f - LINE_WIDTH).unwrap();

    draw.background().color(BLACK);

    let mut rectangles = Vec::<Rectangle>::with_capacity(N_RECTANGLES);
    let rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: size_w,
        height: size_h,
    };
    rectangles.push(rectangle);

    for rectangle in rectangles {
        draw.rect()
            .x(rectangle.x)
            .y(rectangle.y)
            .height(rectangle.height)
            .width(rectangle.width)
            .stroke_weight(LINE_WIDTH)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
