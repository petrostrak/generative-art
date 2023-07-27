use nannou::prelude::*;

const LINE_WIDTH: f32 = 6.0;
// const N_RECTANGLES: usize = 14;

struct Model;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().size(500, 500).view(view).build().unwrap();
    Model
}

struct Rectangle {
    side_a: (Vec2, Vec2),
    side_b: (Vec2, Vec2),
    side_c: (Vec2, Vec2),
    side_d: (Vec2, Vec2),
}

pub struct Rectangles(Vec<Rectangle>);
impl Rectangles {
    fn new() -> Vec<Rectangle> {
        let mut rectangles = Vec::<Rectangle>::new();
        let rectangle = Rectangle {
            side_a: (pt2(-247.0, 244.0), pt2(247.0, 244.0)),
            side_b: (pt2(244.0, 244.0), pt2(244.0, -244.0)),
            side_c: (pt2(247.0, -244.0), pt2(-247.0, -244.0)),
            side_d: (pt2(-244.0, -244.0), pt2(-244.0, 244.0)),
        };
        rectangles.push(rectangle);
        rectangles
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();
    // let size_w = window.w().try_into().map(|f: f32| f - LINE_WIDTH).unwrap();
    // let size_h = window.h().try_into().map(|f: f32| f - LINE_WIDTH).unwrap();

    draw.background().color(WHITE);
    let rectangles = Rectangles::new();

    for rectangle in rectangles {
        draw_rectangle_circumference(&draw, &rectangle);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_rectangle_circumference(draw: &Draw, rectangle: &Rectangle) {
    draw.line()
        .start(rectangle.side_a.0)
        .end(rectangle.side_a.1)
        .weight(LINE_WIDTH)
        .color(BLACK);
    draw.line()
        .start(rectangle.side_b.0)
        .end(rectangle.side_b.1)
        .weight(LINE_WIDTH)
        .color(BLACK);
    draw.line()
        .start(rectangle.side_c.0)
        .end(rectangle.side_c.1)
        .weight(LINE_WIDTH)
        .color(BLACK);
    draw.line()
        .start(rectangle.side_d.0)
        .end(rectangle.side_d.1)
        .weight(LINE_WIDTH)
        .color(BLACK);
}
