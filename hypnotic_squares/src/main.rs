use nannou::prelude::*;

const SIZE: u32 = 320;
const LINE_WIDTH: f32 = 1.2;
const WIDTH: u32 = SIZE;
const HEIGHT: u32 = SIZE;
const FINAL_SIZE: f32 = 10.0;
const START_STEPS: f32 = 4.0;
const OFFSET: f32 = 2.0;
const TILE_STEP: f32 = (SIZE as f32 - OFFSET * 2.0) / 8.0;
const START_SIZE: f32 = TILE_STEP;

fn main() {
    nannou::sketch(view)
        .size(WIDTH, HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}

fn view(app: &App, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw().x_y(-window.w() * 0.45, -window.h() * 0.45);

    draw.background().color(SNOW);

    let directions = vec![-1, 0, 1];
    let mut start_steps: f32;

    for x in (0..=SIZE).step_by(47) {
        for y in (0..=SIZE).step_by(47) {
            start_steps = 2.0 + f32::ceil(random_f32() * 3.0);
            let x_direction =
                directions[f32::floor(random_f32() * directions.len() as f32) as usize];
            let y_direction =
                directions[f32::floor(random_f32() * directions.len() as f32) as usize];
            draw_squares(
                &draw,
                x as f32,
                y as f32,
                START_SIZE,
                START_SIZE,
                x_direction as f32,
                y_direction as f32,
                start_steps - 1.0,
            )
        }
    }

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
        .x_y(x, y)
        .w_h(width, height);

    if steps >= 0.0 {
        let new_size = START_SIZE * (steps / START_STEPS) + FINAL_SIZE;
        let mut new_x: f32 = x + (width - new_size) / 4.0;
        let mut new_y: f32 = y + (height - new_size) / 4.0;
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
