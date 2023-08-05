use nannou::prelude::*;

const ROWS: u32 = 7;
const COLS: u32 = 7;
const SIZE: u32 = 320;
const LINE_WIDTH: f32 = 1.2;
const WIDTH: u32 = 320;
const HEIGHT: u32 = 320;
const FINAL_SIZE: f32 = 10.0;
const START_SIZE: f32 = SIZE as f32;
const START_STEP: f32 = 5.0;

fn main() {
    nannou::sketch(view)
        .size(WIDTH, HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}

fn view(app: &App, frame: Frame) {
    // let window = app.window_rect();
    let draw = app.draw();

    draw.background().color(SNOW);

    // for y in 0..ROWS {
    //     for x in 0..COLS {
    //         let cdraw = gdraw.x_y(x as f32, y as f32);

    //         cdraw
    //             .rect()
    //             .no_fill()
    //             .stroke(BLACK)
    //             .stroke_weight(LINE_WIDTH)
    //             .w_h(1.0, 1.0);
    //     }
    // }

    draw_squares(
        &draw, 0.0, 0.0, START_SIZE, START_SIZE, 0.0, 0.0, START_STEP,
    );

    draw.to_frame(app, &frame).unwrap();
}

fn draw_squares(
    draw: &Draw,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    x_movement: f32,
    y_movement: f32,
    steps: f32,
) {
    draw.rect()
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(LINE_WIDTH)
        // .x_y(x, y)
        .w_h(width, height);

    if steps >= 0.0 {
        let new_size = START_SIZE * (steps / START_STEP) + FINAL_SIZE;
        let mut new_x: f32 = x + (width - new_size) / 2.0;
        let mut new_y = y + (height - new_size) / 2.0;
        new_x = new_x - ((x - new_x) / (steps + 2.0)) * x_movement;
        new_y = new_y - ((y - new_y) / (steps + 2.0)) * y_movement;
        draw_squares(
            &draw,
            new_x,
            new_y,
            new_size,
            new_size,
            x_movement,
            y_movement,
            steps - 1.0,
        );
    }
}
