use nannou::prelude::*;

const ROWS: u32 = 13;
const COLS: u32 = 13;
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

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    let mut odd = false;

    for y in 0..ROWS {
        odd = !odd;
        for x in 0..COLS {
            let cdraw = gdraw.x_y(x as f32, y as f32);

            cdraw
                .rect()
                .x_y(x as f32, y as f32)
                .color(BLACK)
                .w_h(0.1, 0.1);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
