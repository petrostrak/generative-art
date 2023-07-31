use nannou::{
    prelude::*,
    rand::{self, Rng},
};

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
    let size = (window.len() - LINE_WIDTH) as i32;
    let step = (window.len() - LINE_WIDTH) / 6.0;

    draw.background().color(WHITE);

    let mut rectangles = Vec::<Rectangle>::new();
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: window.len() - LINE_WIDTH,
        height: window.len() - LINE_WIDTH,
    };
    rectangles.push(rect);

    for i in (0..=size).step_by(step as usize) {
        split_squares_with(i as f32, 0.0, &mut rectangles);
        split_squares_with(0.0, i as f32, &mut rectangles);
    }

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

fn split_squares_with(x: f32, y: f32, rectangles: &mut Vec<Rectangle>) {
    for (index, rectangle) in rectangles.into_iter().enumerate().rev() {
        if x > 0.0 && x > rectangle.x && x < rectangle.x + rectangle.width {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0..1.0) > 0.5 {
                rectangles.remove(index);
                split_on_x(rectangle, x, rectangles);
            }
        }
        if y > 0.0 && y > rectangle.y && y < rectangle.y + rectangle.height {
            if x > 0.0 && x > rectangle.x && x < rectangle.x + rectangle.width {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0.0..1.0) > 0.5 {
                    rectangles.remove(index);
                    split_on_y(rectangle, y, rectangles);
                }
            }
        }
    }
}

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
