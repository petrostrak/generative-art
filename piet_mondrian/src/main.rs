use nannou::prelude::*;

const LINE_WIDTH: f32 = 8.0;

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
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();

    draw.background().color(WHITE);

    let mut rectangles = Vec::<Rectangle>::new();
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: window.len() - LINE_WIDTH,
        height: window.len() - LINE_WIDTH,
    };
    rectangles.push(rect);

    for rectangle in rectangles {
        draw.rect()
            .no_fill()
            .x_y(rectangle.x, rectangle.y)
            .w_h(rectangle.width, rectangle.height)
            .stroke_weight(LINE_WIDTH)
            .stroke(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn split_squares_with(x: f32, y: f32, rectangles: &mut Vec<Rectangle>) {}

fn split_on_x(rectangle: &Rectangle, split_at: f32, rectangles: &mut Vec<Rectangle>) {
    let rectangle_a = Rectangle {
        x: rectangle.x,
        y: rectangle.y,
        width: rectangle.width - (rectangle.width - split_at + rectangle.x),
        height: rectangle.height,
    };

    let rectangle_b = Rectangle {
        x: split_at,
        y: rectangle.y,
        width: rectangle.width - split_at + rectangle.x,
        height: rectangle.height,
    };

    rectangles.push(rectangle_a);
    rectangles.push(rectangle_b);
}

fn split_on_y(rectangle: &Rectangle, split_at: f32, rectangles: &mut Vec<Rectangle>) {
    let rectangle_a = Rectangle {
        x: rectangle.x,
        y: rectangle.y,
        width: rectangle.width,
        height: rectangle.height - (rectangle.height - split_at + rectangle.y),
    };

    let rectangle_b = Rectangle {
        x: rectangle.x,
        y: split_at,
        width: rectangle.width,
        height: rectangle.height - split_at + rectangle.y,
    };

    rectangles.push(rectangle_a);
    rectangles.push(rectangle_b);
}
