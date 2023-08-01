use nannou::prelude::*;
use rand::Rng;

const LINE_WIDTH: f32 = 6.0;

struct Model;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().size(320, 320).view(view).build().unwrap();
    Model
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let size = window.len();
    let draw = app.draw().x_y(-window.w() * 0.41, -window.h() * 0.41);

    let step = size / 6.0;

    draw.background().color(WHITE);

    let mut rectangles = Vec::<Rectangle>::new();
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: size,
        height: size,
    };
    rectangles.push(rect);

    for i in (0..size as usize).step_by(step as usize) {
        split_squares_with(0.0, i as f32, &mut rectangles);
        split_squares_with(i as f32, 0.0, &mut rectangles);
    }

    for rectangle in rectangles {
        println!("{:?}", rectangle);
        draw.rect()
            .no_fill()
            .x(rectangle.x)
            .y(rectangle.y)
            .w_h(rectangle.width, rectangle.height)
            .stroke_weight(LINE_WIDTH)
            .stroke(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn split_squares_with(x: f32, y: f32, rectangles: &mut Vec<Rectangle>) {
    for i in (0..rectangles.len()).rev() {
        let rectangle = rectangles[i];
        if x > 0.0 && x > rectangle.x && x < rectangle.x + rectangle.width {
            // let rng = rand::thread_rng().gen_range(0.0..1.0);
            // if rng > 0.5 {
            rectangles.splice(i..i, split_on_x(rectangle, x));
            rectangles.remove(i + 2);
            // }
        }
        if y > 0.0 && y > rectangle.y && y < rectangle.y + rectangle.height {
            // let rng = rand::thread_rng().gen_range(0.0..1.0);
            // if rng > 0.5 {
            rectangles.splice(i..i, split_on_y(rectangle, y));
            rectangles.remove(i + 2);
            // }
        }
    }
}

fn split_on_x(rectangle: Rectangle, split_at: f32) -> Vec<Rectangle> {
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

    vec![rectangle_a, rectangle_b]
}

fn split_on_y(rectangle: Rectangle, split_at: f32) -> Vec<Rectangle> {
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

    vec![rectangle_a, rectangle_b]
}
