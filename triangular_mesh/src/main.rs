use nannou::prelude::*;

const ROWS: u32 = 16;
const COLS: u32 = 16;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::sketch(view)
        .size(WIDTH, HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}

struct Dot {
    x: f32,
    y: f32,
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    let mut odd = false;
    let mut dots = Vec::<Dot>::new();

    for y in 0..ROWS {
        odd = !odd;
        for x in 1..COLS {
            let dot = Dot {
                x: x as f32 + (if odd { x as f32 / 2.0 } else { 0.0 }),
                y: y as f32,
            };
            dots.push(dot);
        }
    }

    for dot in dots {
        let cdraw = gdraw.x_y(dot.x, dot.y);

        cdraw.rect().x_y(dot.x, dot.y).color(BLACK).w_h(0.1, 0.1);
    }

    draw.to_frame(app, &frame).unwrap();
}
