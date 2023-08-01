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
struct Square {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let size = window.len();
    let draw = app.draw();

    let step = size / 6.0;

    draw.background().color(WHITE);

    let mut squares = Vec::<Square>::new();
    let sq = Square {
        x: 0.0,
        y: 0.0,
        width: size,
        height: size,
    };
    squares.push(sq);
    println!("Initial square: {:?}", sq);

    for i in (0..size as usize).step_by(step as usize) {
        split_squares_with(0.0, i as f32, &mut squares);
        split_squares_with(i as f32, 0.0, &mut squares);
    }

    for square in squares {
        println!("{:?}", square);
        draw.rect()
            .no_fill()
            .x_y(square.x, square.y)
            .w_h(square.width, square.height)
            .stroke_weight(LINE_WIDTH)
            .stroke(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn split_squares_with(x: f32, y: f32, squares: &mut Vec<Square>) {
    for i in (0..squares.len()).rev() {
        let square = squares[i];
        if x > 0.0 && x > square.x && x < square.x + square.width {
            if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {
                squares.splice(i..i + 1, split_on_x(square, x));
            }
        }
        if y > 0.0 && y > square.y && y < square.y + square.height {
            if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {
                squares.splice(i..i + 1, split_on_y(square, y));
            }
        }
    }
}

fn split_on_x(square: Square, split_at: f32) -> Vec<Square> {
    let square_a = Square {
        x: square.x,
        y: square.y,
        width: square.width - (square.width - split_at + square.x),
        height: square.height,
    };

    let square_b = Square {
        x: split_at,
        y: square.y,
        width: square.width - split_at + square.x,
        height: square.height,
    };

    vec![square_a, square_b]
}

fn split_on_y(square: Square, split_at: f32) -> Vec<Square> {
    let square_a = Square {
        x: square.x,
        y: square.y,
        width: square.width,
        height: square.height - (square.height - split_at + square.y),
    };

    let square_b = Square {
        x: square.x,
        y: split_at,
        width: square.width,
        height: square.height - split_at + square.y,
    };

    vec![square_a, square_b]
}
